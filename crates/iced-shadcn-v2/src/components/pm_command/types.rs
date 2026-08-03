//! Public configuration and command-resolution types for [`super::PmCommand`].

use std::fmt;

/// A package manager displayed as an agent tab.
///
/// The named variants cover the package managers understood by the reference
/// `package-manager-detector` resolver. [`Self::Custom`] keeps the component
/// useful with a project-specific executable as well.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PmCommandAgent {
    /// The npm client.
    #[default]
    Npm,
    /// The pnpm client.
    Pnpm,
    /// The legacy pnpm 6 client, whose `run` command uses a `--` separator.
    Pnpm6,
    /// The Yarn classic client.
    Yarn,
    /// The Bun client.
    Bun,
    /// The Deno client.
    Deno,
    /// Yarn Berry, accepted by the detector as a Yarn agent with Berry
    /// command semantics.
    YarnBerry,
    /// An arbitrary package-manager executable or detector agent name.
    Custom(String),
}

impl PmCommandAgent {
    /// Returns the spelling used in the tab label and resolver input.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::Pnpm6 => "pnpm@6",
            Self::Yarn => "yarn",
            Self::Bun => "bun",
            Self::Deno => "deno",
            Self::YarnBerry => "yarn@berry",
            Self::Custom(agent) => agent,
        }
    }

    /// Returns the four agents used by the Svelte component by default.
    #[must_use]
    pub fn defaults() -> Vec<Self> {
        vec![Self::Npm, Self::Pnpm, Self::Yarn, Self::Bun]
    }

    /// Returns `true` when the agent uses Yarn Berry command semantics.
    #[must_use]
    pub fn is_yarn_berry(&self) -> bool {
        matches!(self, Self::YarnBerry)
            || matches!(self, Self::Custom(value) if {
                let value = value.to_ascii_lowercase();
                value == "yarn@berry" || value == "yarn-berry" || value == "yarn_berry"
            })
    }
}

impl fmt::Display for PmCommandAgent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&str> for PmCommandAgent {
    fn from(agent: &str) -> Self {
        match agent.to_ascii_lowercase().as_str() {
            "npm" => Self::Npm,
            "pnpm" => Self::Pnpm,
            "pnpm@6" => Self::Pnpm6,
            "yarn" => Self::Yarn,
            "bun" => Self::Bun,
            "deno" => Self::Deno,
            "yarn@berry" | "yarn-berry" | "yarn_berry" => Self::YarnBerry,
            _ => Self::Custom(agent.to_owned()),
        }
    }
}

impl From<String> for PmCommandAgent {
    fn from(agent: String) -> Self {
        Self::from(agent.as_str())
    }
}

/// A package-manager command understood by the reference resolver.
///
/// [`Self::Custom`] is intentionally available for a custom agent or a
/// future detector command. Known agents retain their package-manager-specific
/// aliases such as `npm i`, `pnpm dlx`, and `yarn up`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PmCommandVerb {
    /// Run a package script.
    Run,
    /// Install project dependencies.
    Install,
    /// Install from a lockfile without updating it.
    Frozen,
    /// Install a package globally.
    Global,
    /// Add a dependency.
    Add,
    /// Upgrade dependencies.
    Upgrade,
    /// Upgrade dependencies interactively.
    UpgradeInteractive,
    /// Deduplicate dependencies.
    Dedupe,
    /// Execute a package through the package manager's one-shot runner.
    Execute,
    /// Execute a locally installed package.
    ExecuteLocal,
    /// Remove a dependency.
    Uninstall,
    /// Remove a globally installed dependency.
    GlobalUninstall,
    /// An arbitrary detector command name.
    Custom(String),
}

impl PmCommandVerb {
    /// Returns the detector spelling for this command.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Run => "run",
            Self::Install => "install",
            Self::Frozen => "frozen",
            Self::Global => "global",
            Self::Add => "add",
            Self::Upgrade => "upgrade",
            Self::UpgradeInteractive => "upgrade-interactive",
            Self::Dedupe => "dedupe",
            Self::Execute => "execute",
            Self::ExecuteLocal => "execute-local",
            Self::Uninstall => "uninstall",
            Self::GlobalUninstall => "global_uninstall",
            Self::Custom(command) => command,
        }
    }
}

impl fmt::Display for PmCommandVerb {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&str> for PmCommandVerb {
    fn from(command: &str) -> Self {
        match command.to_ascii_lowercase().as_str() {
            "run" => Self::Run,
            "install" => Self::Install,
            "frozen" => Self::Frozen,
            "global" => Self::Global,
            "add" => Self::Add,
            "upgrade" => Self::Upgrade,
            "upgrade-interactive" => Self::UpgradeInteractive,
            "dedupe" => Self::Dedupe,
            "execute" => Self::Execute,
            "execute-local" => Self::ExecuteLocal,
            "uninstall" => Self::Uninstall,
            "global_uninstall" => Self::GlobalUninstall,
            _ => Self::Custom(command.to_owned()),
        }
    }
}

impl From<String> for PmCommandVerb {
    fn from(command: String) -> Self {
        Self::from(command.as_str())
    }
}

/// The visual treatment of a [`super::PmCommand`] root.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PmCommandVariant {
    /// A card surface with the theme border.
    #[default]
    Default,
    /// A half-opacity secondary surface with a transparent outer border.
    Secondary,
}

/// Corner-radius choice for a [`super::PmCommand`] root.
///
/// The default inherits the active button recipe. This is what lets PMCommand
/// follow style packs that do not define a PMCommand-specific recipe.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PmCommandRadius {
    /// Inherit the active style pack's button radius.
    #[default]
    Default,
    /// No rounding.
    None,
    /// The active style pack's small radius.
    Small,
    /// The active style pack's medium radius.
    Medium,
    /// The active style pack's large radius.
    Large,
    /// A fully rounded root.
    Full,
}

/// A resolved executable and argument vector for a package-manager command.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PmCommandResolution {
    command: String,
    args: Vec<String>,
}

impl PmCommandResolution {
    /// Creates a resolved command from an executable and argument vector.
    #[must_use]
    pub fn new(
        command: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    /// Returns the executable portion of the resolved command.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns the resolved arguments in order.
    #[must_use]
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Returns the exact unquoted display string used by PMCommand.
    ///
    /// This deliberately joins arguments without shell quoting, matching the
    /// Svelte reference's visible command text. Empty argument lists do not
    /// add trailing whitespace.
    #[must_use]
    pub fn command_text(&self) -> String {
        if self.args.is_empty() {
            self.command.clone()
        } else {
            format!("{} {}", self.command, self.args.join(" "))
        }
    }

    /// Consumes the resolution and returns `(executable, args)`.
    #[must_use]
    pub fn into_parts(self) -> (String, Vec<String>) {
        (self.command, self.args)
    }
}

/// Resolves a package-manager command using the same aliases as
/// `package-manager-detector`.
#[must_use]
pub fn resolve_pm_command(
    agent: &PmCommandAgent,
    command: &PmCommandVerb,
    args: &[String],
) -> PmCommandResolution {
    try_resolve_pm_command(agent, command, args).unwrap_or_else(|| {
        let mut fallback = vec![command.as_str().to_owned()];
        fallback.extend(args.iter().cloned());
        PmCommandResolution::new(agent.as_str(), fallback)
    })
}

/// Attempts to resolve a package-manager command exactly as the reference
/// detector does.
///
/// Some detector combinations intentionally return no command (for example,
/// npm's `upgrade-interactive`). Use this function when an application needs
/// to distinguish that unsupported combination from the display-friendly
/// fallback returned by [`resolve_pm_command`].
#[must_use]
pub fn try_resolve_pm_command(
    agent: &PmCommandAgent,
    command: &PmCommandVerb,
    args: &[String],
) -> Option<PmCommandResolution> {
    let resolution = match agent {
        PmCommandAgent::Npm => resolve_npm(command, args),
        PmCommandAgent::Pnpm => resolve_pnpm(command, args),
        PmCommandAgent::Pnpm6 => resolve_pnpm6(command, args),
        PmCommandAgent::Yarn => resolve_yarn(command, args, false),
        PmCommandAgent::YarnBerry => resolve_yarn(command, args, true),
        PmCommandAgent::Bun => resolve_bun(command, args),
        PmCommandAgent::Deno => resolve_deno(command, args),
        PmCommandAgent::Custom(agent) => resolve_custom(agent, command, args),
    };

    resolution.map(|(command, args)| PmCommandResolution::new(command, args))
}

fn resolve_npm(command: &PmCommandVerb, args: &[String]) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Execute | PmCommandVerb::ExecuteLocal => append("npx", [], args),
        PmCommandVerb::Run => append_dash_dash("npm", ["run"], args),
        PmCommandVerb::Install | PmCommandVerb::Add => append("npm", ["i"], args),
        PmCommandVerb::Frozen => append("npm", ["ci"], args),
        PmCommandVerb::Global => append("npm", ["i", "-g"], args),
        PmCommandVerb::Upgrade => append("npm", ["update"], args),
        PmCommandVerb::Dedupe => append("npm", ["dedupe"], args),
        PmCommandVerb::Uninstall => append("npm", ["uninstall"], args),
        PmCommandVerb::GlobalUninstall => append("npm", ["uninstall", "-g"], args),
        PmCommandVerb::UpgradeInteractive => None,
        PmCommandVerb::Custom(command) => append("npm", [command.as_str()], args),
    }
}

fn resolve_pnpm(command: &PmCommandVerb, args: &[String]) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Run => append("pnpm", ["run"], args),
        PmCommandVerb::Install => append("pnpm", ["i"], args),
        PmCommandVerb::Frozen => append("pnpm", ["i", "--frozen-lockfile"], args),
        PmCommandVerb::Global => append("pnpm", ["add", "-g"], args),
        PmCommandVerb::Add => append("pnpm", ["add"], args),
        PmCommandVerb::Upgrade => append("pnpm", ["update"], args),
        PmCommandVerb::UpgradeInteractive => append("pnpm", ["update", "-i"], args),
        PmCommandVerb::Dedupe => append("pnpm", ["dedupe"], args),
        PmCommandVerb::Execute => append("pnpm", ["dlx"], args),
        PmCommandVerb::ExecuteLocal => append("pnpm", ["exec"], args),
        PmCommandVerb::Uninstall => append("pnpm", ["remove"], args),
        PmCommandVerb::GlobalUninstall => append("pnpm", ["remove", "--global"], args),
        PmCommandVerb::Custom(command) => append("pnpm", [command.as_str()], args),
    }
}

fn resolve_pnpm6(command: &PmCommandVerb, args: &[String]) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Run => append_dash_dash("pnpm", ["run"], args),
        _ => resolve_pnpm(command, args),
    }
}

fn resolve_yarn(
    command: &PmCommandVerb,
    args: &[String],
    berry: bool,
) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Execute if !berry => append("npx", [], args),
        PmCommandVerb::Execute => append("yarn", ["dlx"], args),
        PmCommandVerb::ExecuteLocal => {
            let mut resolved = vec!["exec".to_owned()];
            if let Some((first, rest)) = args.split_first() {
                resolved.push(first.clone());
                if !rest.is_empty() {
                    resolved.push("--".to_owned());
                    resolved.extend(rest.iter().cloned());
                }
            }
            Some(("yarn".to_owned(), resolved))
        }
        PmCommandVerb::Run => append("yarn", ["run"], args),
        PmCommandVerb::Install if berry => append("yarn", ["install"], args),
        PmCommandVerb::Install => append("yarn", ["install"], args),
        PmCommandVerb::Frozen if berry => append("yarn", ["install", "--immutable"], args),
        PmCommandVerb::Frozen => append("yarn", ["install", "--frozen-lockfile"], args),
        PmCommandVerb::Global if berry => append("npm", ["i", "-g"], args),
        PmCommandVerb::Global => append("yarn", ["global", "add"], args),
        PmCommandVerb::Add => append("yarn", ["add"], args),
        PmCommandVerb::Upgrade if berry => append("yarn", ["up"], args),
        PmCommandVerb::Upgrade => append("yarn", ["upgrade"], args),
        PmCommandVerb::UpgradeInteractive if berry => append("yarn", ["up", "-i"], args),
        PmCommandVerb::UpgradeInteractive => append("yarn", ["upgrade-interactive"], args),
        PmCommandVerb::Dedupe if berry => append("yarn", ["dedupe"], args),
        PmCommandVerb::Dedupe => None,
        PmCommandVerb::Uninstall => append("yarn", ["remove"], args),
        PmCommandVerb::GlobalUninstall if berry => append("npm", ["uninstall", "-g"], args),
        PmCommandVerb::GlobalUninstall => append("yarn", ["global", "remove"], args),
        PmCommandVerb::Custom(command) => append("yarn", [command.as_str()], args),
    }
}

fn resolve_bun(command: &PmCommandVerb, args: &[String]) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Run => append("bun", ["run"], args),
        PmCommandVerb::Install => append("bun", ["install"], args),
        PmCommandVerb::Frozen => append("bun", ["install", "--frozen-lockfile"], args),
        PmCommandVerb::Global => append("bun", ["add", "-g"], args),
        PmCommandVerb::Add => append("bun", ["add"], args),
        PmCommandVerb::Upgrade => append("bun", ["update"], args),
        PmCommandVerb::UpgradeInteractive => append("bun", ["update", "-i"], args),
        PmCommandVerb::Dedupe => None,
        PmCommandVerb::Execute | PmCommandVerb::ExecuteLocal => append("bun", ["x"], args),
        PmCommandVerb::Uninstall => append("bun", ["remove"], args),
        PmCommandVerb::GlobalUninstall => append("bun", ["remove", "-g"], args),
        PmCommandVerb::Custom(command) => append("bun", [command.as_str()], args),
    }
}

fn resolve_deno(command: &PmCommandVerb, args: &[String]) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Run => append("deno", ["task"], args),
        PmCommandVerb::Execute => {
            let mut resolved = vec!["run".to_owned()];
            if let Some((first, rest)) = args.split_first() {
                resolved.push(format!("npm:{first}"));
                resolved.extend(rest.iter().cloned());
            }
            Some(("deno".to_owned(), resolved))
        }
        PmCommandVerb::ExecuteLocal => append("deno", ["task", "--eval"], args),
        PmCommandVerb::Install => append("deno", ["install"], args),
        PmCommandVerb::Add => append("deno", ["add"], args),
        PmCommandVerb::Frozen => append("deno", ["install", "--frozen"], args),
        PmCommandVerb::Global => append("deno", ["install", "-g"], args),
        PmCommandVerb::Upgrade | PmCommandVerb::UpgradeInteractive => {
            append("deno", ["outdated", "--update"], args)
        }
        PmCommandVerb::Dedupe => None,
        PmCommandVerb::Uninstall => append("deno", ["remove"], args),
        PmCommandVerb::GlobalUninstall => append("deno", ["remove", "--global"], args),
        PmCommandVerb::Custom(command) => append("deno", [command.as_str()], args),
    }
}

fn resolve_custom(
    agent: &str,
    command: &PmCommandVerb,
    args: &[String],
) -> Option<(String, Vec<String>)> {
    match command {
        PmCommandVerb::Execute | PmCommandVerb::ExecuteLocal => {
            Some((agent.to_owned(), args.to_vec()))
        }
        _ => append(agent, [command.as_str()], args),
    }
}

fn append<const N: usize>(
    command: &str,
    prefix: [&str; N],
    args: &[String],
) -> Option<(String, Vec<String>)> {
    let mut resolved = Vec::with_capacity(N + args.len());
    resolved.extend(prefix.into_iter().map(str::to_owned));
    resolved.extend(args.iter().cloned());
    Some((command.to_owned(), resolved))
}

fn append_dash_dash<const N: usize>(
    command: &str,
    prefix: [&str; N],
    args: &[String],
) -> Option<(String, Vec<String>)> {
    let mut resolved = Vec::with_capacity(N + args.len() + usize::from(args.len() > 1));
    resolved.extend(prefix.into_iter().map(str::to_owned));
    if let Some((first, rest)) = args.split_first() {
        resolved.push(first.clone());
        if !rest.is_empty() {
            resolved.push("--".to_owned());
            resolved.extend(rest.iter().cloned());
        }
    }
    Some((command.to_owned(), resolved))
}
