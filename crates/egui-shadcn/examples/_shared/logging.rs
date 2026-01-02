use std::fs::{File, create_dir_all};
use std::io;
use std::path::{Path, PathBuf};

fn default_log_path(example_name: &str) -> PathBuf {
    Path::new(".context")
        .join("logs")
        .join(format!("{example_name}.log"))
}

/// Initialize `env_logger` and write logs to a file.
///
/// If `SHADCN_LOG_PATH` is set, logs are written there; otherwise defaults to
/// `.context/logs/{example}.log` (relative to current working directory).
pub fn init_file_logger(example_name: &str) -> io::Result<PathBuf> {
    let path = std::env::var_os("SHADCN_LOG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| default_log_path(example_name));

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let file = File::create(&path)?;

    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env)
        .target(env_logger::Target::Pipe(Box::new(file)))
        .format_timestamp_millis()
        .is_test(false)
        .try_init()
        .ok();

    Ok(path)
}
