use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Table, value};

const CONFIG_FILE: &str = "shadcn-rs.toml";
const CARGO_FILE: &str = "Cargo.toml";

const EGUI_CRATE_NAME: &str = "egui-shadcn";
const ICED_CRATE_NAME: &str = "iced-shadcn";
const EGUI_IMPORT_PREFIX: &str = "egui_shadcn";
const ICED_IMPORT_PREFIX: &str = "iced_shadcn";

const SOURCE_MODULE_EXCLUDE: &[&str] = &[
    "lib",
    "theme",
    "tokens",
    "menu_primitives",
    "overlay",
    "new_api",
    "accent_palette",
    "icons",
];

#[derive(Parser, Debug)]
#[command(name = "shadcn-rs")]
#[command(about = "Install shadcn-rs components into your Rust project")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init(InitArgs),
    List(ListArgs),
    Add(AddArgs),
}

#[derive(clap::Args, Debug)]
struct InitArgs {
    #[arg(long, default_value = ".")]
    project: PathBuf,
    #[arg(long, default_value = "src/shadcn")]
    target_dir: String,
    #[arg(long)]
    backend: Option<Backend>,
}

#[derive(clap::Args, Debug)]
struct ListArgs {
    #[arg(long, default_value = ".")]
    project: PathBuf,
    #[arg(long)]
    backend: Option<Backend>,
}

#[derive(clap::Args, Debug)]
struct AddArgs {
    component: String,
    #[arg(long, default_value = ".")]
    project: PathBuf,
    #[arg(long)]
    backend: Option<Backend>,
    #[arg(long)]
    write_cargo: bool,
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Egui,
    Iced,
}

impl Backend {
    fn as_crate_dependency(self) -> &'static str {
        match self {
            Self::Egui => EGUI_CRATE_NAME,
            Self::Iced => ICED_CRATE_NAME,
        }
    }

    fn import_prefix(self) -> &'static str {
        match self {
            Self::Egui => EGUI_IMPORT_PREFIX,
            Self::Iced => ICED_IMPORT_PREFIX,
        }
    }

    fn as_folder(self) -> &'static str {
        match self {
            Self::Egui => "egui",
            Self::Iced => "iced",
        }
    }

    fn src_root(self, workspace_root: &Path) -> PathBuf {
        workspace_root
            .join("crates")
            .join(self.as_crate_dependency())
            .join("src")
    }
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Egui => write!(f, "egui"),
            Self::Iced => write!(f, "iced"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallerConfig {
    target_dir: String,
    #[serde(default)]
    default_backend: Option<Backend>,
}

#[derive(Debug, Clone)]
struct ComponentEntry {
    slug: String,
    source_file: PathBuf,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args) => cmd_init(args),
        Commands::List(args) => cmd_list(args),
        Commands::Add(args) => cmd_add(args),
    }
}

fn cmd_init(args: InitArgs) -> Result<()> {
    let project_root = find_project_root(&args.project)?;
    let config_path = project_root.join(CONFIG_FILE);

    let config = InstallerConfig {
        target_dir: args.target_dir,
        default_backend: args.backend,
    };
    let encoded = toml::to_string_pretty(&config).context("failed to encode config TOML")?;
    fs::write(&config_path, encoded)
        .with_context(|| format!("failed to write {}", config_path.display()))?;

    let target_root = project_root.join(&config.target_dir);
    fs::create_dir_all(&target_root)
        .with_context(|| format!("failed to create {}", target_root.display()))?;

    println!(
        "Initialized shadcn-rs config at {}",
        config_path.to_string_lossy()
    );
    Ok(())
}

fn cmd_list(args: ListArgs) -> Result<()> {
    let workspace_root = shadcn_workspace_root()?;
    match args.backend {
        Some(backend) => {
            let components = collect_components(&workspace_root, backend)?;
            print_component_list(backend, &components);
        }
        None => {
            for backend in [Backend::Egui, Backend::Iced] {
                let components = collect_components(&workspace_root, backend)?;
                print_component_list(backend, &components);
            }
        }
    }
    let _ = args.project;
    Ok(())
}

fn print_component_list(backend: Backend, components: &[ComponentEntry]) {
    println!("{backend}:");
    for component in components {
        println!("  - {}", component.slug);
    }
}

fn cmd_add(args: AddArgs) -> Result<()> {
    let project_root = find_project_root(&args.project)?;
    let config = load_config(&project_root)?;
    let cargo_toml_path = project_root.join(CARGO_FILE);
    let cargo_toml_raw = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("failed to read {}", cargo_toml_path.display()))?;
    let backend = resolve_backend(
        args.backend,
        config.default_backend,
        &cargo_toml_raw,
        &cargo_toml_path,
    )?;

    let workspace_root = shadcn_workspace_root()?;
    let components = collect_components(&workspace_root, backend)?;
    let slug = sanitize_slug(&args.component);
    let entry = components
        .iter()
        .find(|component| component.slug == slug)
        .ok_or_else(|| anyhow!("component '{slug}' was not found for backend '{backend}'"))?;

    let source = fs::read_to_string(&entry.source_file)
        .with_context(|| format!("failed to read {}", entry.source_file.display()))?;
    let rewritten = rewrite_component_source(&source, backend.import_prefix());
    ensure_rewrite_is_valid(&rewritten)?;

    let target_base = project_root
        .join(&config.target_dir)
        .join(backend.as_folder());
    fs::create_dir_all(&target_base)
        .with_context(|| format!("failed to create {}", target_base.display()))?;

    let target_file = target_base.join(format!("{slug}.rs"));
    if target_file.exists() && !args.force {
        bail!(
            "component file already exists: {} (use --force to overwrite)",
            target_file.display()
        );
    }
    fs::write(&target_file, rewritten)
        .with_context(|| format!("failed to write {}", target_file.display()))?;

    write_mod_files(&project_root.join(&config.target_dir), backend)?;

    let dependency = backend.as_crate_dependency();
    if has_dependency(&cargo_toml_raw, dependency)? {
        println!("Dependency '{dependency}' already exists in Cargo.toml");
    } else if args.write_cargo {
        add_dependency_to_manifest(&cargo_toml_path, dependency, "0.5.0")?;
        println!("Added dependency '{dependency} = \"0.5.0\"' to Cargo.toml");
    } else {
        println!(
            "Dependency '{dependency}' is missing in Cargo.toml. Re-run with --write-cargo to add it automatically."
        );
    }

    println!(
        "Installed component '{slug}' to {}",
        target_file.to_string_lossy()
    );
    Ok(())
}

fn sanitize_slug(input: &str) -> String {
    input.trim().replace('-', "_")
}

fn load_config(project_root: &Path) -> Result<InstallerConfig> {
    let config_path = project_root.join(CONFIG_FILE);
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    let config: InstallerConfig = toml::from_str(&content)
        .with_context(|| format!("invalid config in {}", config_path.display()))?;
    Ok(config)
}

fn resolve_backend(
    explicit: Option<Backend>,
    config_default: Option<Backend>,
    cargo_toml: &str,
    manifest_path: &Path,
) -> Result<Backend> {
    if let Some(backend) = explicit {
        return Ok(backend);
    }
    if let Some(backend) = config_default {
        return Ok(backend);
    }

    let has_egui = has_dependency(cargo_toml, EGUI_CRATE_NAME)?;
    let has_iced = has_dependency(cargo_toml, ICED_CRATE_NAME)?;
    match (has_egui, has_iced) {
        (true, false) => Ok(Backend::Egui),
        (false, true) => Ok(Backend::Iced),
        (false, false) => bail!(
            "cannot detect backend from {}: add '{}' or '{}' dependency, or pass --backend",
            manifest_path.display(),
            EGUI_CRATE_NAME,
            ICED_CRATE_NAME
        ),
        (true, true) => bail!(
            "both '{}' and '{}' found in {}: pass --backend explicitly",
            EGUI_CRATE_NAME,
            ICED_CRATE_NAME,
            manifest_path.display()
        ),
    }
}

fn find_project_root(candidate: &Path) -> Result<PathBuf> {
    let root = candidate
        .canonicalize()
        .with_context(|| format!("failed to resolve project path {}", candidate.display()))?;
    let cargo_toml = root.join(CARGO_FILE);
    if !cargo_toml.is_file() {
        bail!(
            "project root must contain {}: {}",
            CARGO_FILE,
            cargo_toml.display()
        );
    }
    Ok(root)
}

fn shadcn_workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| anyhow!("failed to resolve shadcn-rs workspace root"))?
        .to_path_buf();
    if !workspace_root.join(CARGO_FILE).is_file() {
        bail!(
            "workspace root does not contain {}: {}",
            CARGO_FILE,
            workspace_root.display()
        );
    }
    Ok(workspace_root)
}

fn collect_components(workspace_root: &Path, backend: Backend) -> Result<Vec<ComponentEntry>> {
    let src_root = backend.src_root(workspace_root);
    let lib_path = src_root.join("lib.rs");
    let lib_source = fs::read_to_string(&lib_path)
        .with_context(|| format!("failed to read {}", lib_path.display()))?;
    let mut entries = Vec::new();
    for module in parse_pub_mods(&lib_source) {
        if SOURCE_MODULE_EXCLUDE.contains(&module.as_str()) {
            continue;
        }
        let module_file = src_root.join(format!("{module}.rs"));
        if module_file.is_file() {
            entries.push(ComponentEntry {
                slug: module,
                source_file: module_file,
            });
        }
    }

    entries.sort_by(|left, right| left.slug.cmp(&right.slug));
    entries.dedup_by(|left, right| left.slug == right.slug);
    Ok(entries)
}

fn parse_pub_mods(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("pub mod ") || !line.ends_with(';') {
                return None;
            }
            let name = line.trim_start_matches("pub mod ").trim_end_matches(';');
            if name.is_empty() {
                return None;
            }
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return None;
            }
            Some(name.to_string())
        })
        .collect()
}

fn rewrite_component_source(source: &str, import_prefix: &str) -> String {
    source
        .replace("use crate::", &format!("use {import_prefix}::"))
        .replace("crate::", &format!("{import_prefix}::"))
}

fn ensure_rewrite_is_valid(rewritten: &str) -> Result<()> {
    if rewritten.contains("crate::") {
        bail!(
            "component still contains internal 'crate::' paths after rewrite; this module is unsupported for single-file install"
        );
    }
    Ok(())
}

fn write_mod_files(target_root: &Path, backend: Backend) -> Result<()> {
    fs::create_dir_all(target_root)
        .with_context(|| format!("failed to create {}", target_root.display()))?;

    let backend_dir = target_root.join(backend.as_folder());
    fs::create_dir_all(&backend_dir)
        .with_context(|| format!("failed to create {}", backend_dir.display()))?;

    let shadcn_mod = target_root.join("mod.rs");
    let mut backend_dirs = BTreeSet::new();
    for entry in fs::read_dir(target_root)
        .with_context(|| format!("failed to read {}", target_root.display()))?
    {
        let entry = entry?;
        if entry.path().is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            backend_dirs.insert(name.to_string());
        }
    }
    let mut shadcn_mod_content = String::new();
    for dir in backend_dirs {
        shadcn_mod_content.push_str(&format!("pub mod {dir};\n"));
    }
    fs::write(&shadcn_mod, shadcn_mod_content)
        .with_context(|| format!("failed to write {}", shadcn_mod.display()))?;

    let backend_mod = backend_dir.join("mod.rs");
    let mut modules = BTreeSet::new();
    for entry in fs::read_dir(&backend_dir)
        .with_context(|| format!("failed to read {}", backend_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");
        if stem == "mod" || stem.is_empty() {
            continue;
        }
        modules.insert(stem.to_string());
    }
    let mut backend_mod_content = String::new();
    for module in modules {
        backend_mod_content.push_str(&format!("pub mod {module};\n"));
    }
    fs::write(&backend_mod, backend_mod_content)
        .with_context(|| format!("failed to write {}", backend_mod.display()))?;
    Ok(())
}

fn has_dependency(cargo_toml: &str, dependency_name: &str) -> Result<bool> {
    let doc = cargo_toml
        .parse::<DocumentMut>()
        .context("failed to parse Cargo.toml")?;
    Ok(doc
        .as_table()
        .get("dependencies")
        .and_then(Item::as_table_like)
        .map(|deps| deps.contains_key(dependency_name))
        .unwrap_or(false))
}

fn add_dependency_to_manifest(
    manifest_path: &Path,
    dependency_name: &str,
    version: &str,
) -> Result<()> {
    let raw = fs::read_to_string(manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let mut doc = raw
        .parse::<DocumentMut>()
        .with_context(|| format!("failed to parse {}", manifest_path.display()))?;

    if doc["dependencies"].is_none() {
        doc["dependencies"] = Item::Table(Table::new());
    }
    let deps = doc["dependencies"]
        .as_table_like_mut()
        .ok_or_else(|| anyhow!("dependencies section is not a TOML table"))?;
    if !deps.contains_key(dependency_name) {
        deps.insert(dependency_name, value(version));
    }

    fs::write(manifest_path, doc.to_string())
        .with_context(|| format!("failed to write {}", manifest_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pub_mods_extracts_modules() {
        let source = r#"
            pub mod button;
            pub mod sidebar;
            mod private;
            pub mod chart;
        "#;
        let modules = parse_pub_mods(source);
        assert_eq!(modules, vec!["button", "sidebar", "chart"]);
    }

    #[test]
    fn rewrite_component_source_rewrites_crate_imports() {
        let source = r#"
            use crate::theme::Theme;
            let x = crate::button::button();
        "#;
        let rewritten = rewrite_component_source(source, "egui_shadcn");
        assert!(rewritten.contains("use egui_shadcn::theme::Theme;"));
        assert!(rewritten.contains("egui_shadcn::button::button();"));
        assert!(!rewritten.contains("crate::"));
    }

    #[test]
    fn has_dependency_detects_dependency() {
        let cargo = r#"
            [package]
            name = "demo"
            version = "0.1.0"

            [dependencies]
            iced-shadcn = "0.5.0"
        "#;
        assert!(has_dependency(cargo, "iced-shadcn").expect("parse dependency"));
        assert!(!has_dependency(cargo, "egui-shadcn").expect("parse dependency"));
    }
}
