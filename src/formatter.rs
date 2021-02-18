//! The central piece. This represents an individual code formatter following the treefmt formatter spec.
use crate::config;
use crate::CLOG;
use anyhow::Result;
use std::path::PathBuf;
use std::process::{Command, Output};
use which::which;

/// Formatter includes all the formatter configuration.
pub struct Formatter {
    command: PathBuf,
    options: Vec<String>,
    work_dir: PathBuf,
    includes: Vec<String>,
    excludes: Vec<String>,
}

impl Formatter {
    /// Invoke the formatter on the given paths.
    fn format(self: Formatter, paths: Vec<String>) -> Result<Output, std::io::Error> {
        let mut args: Vec<String> = self.options.clone();
        args.extend(paths);
        Command::new(self.command)
            .args(&args)
            .current_dir(self.work_dir)
            .output()
    }

    /// Returns true if given path matches one or more of the includes, and none of the excludes.
    fn check(_path: &PathBuf) -> bool {
        // TODO
        true
    }
}

/// Converts config to formatter
///
/// config_dir is the folder where the treefmt.toml was found.
/// fmt is a fragment of that configuration
pub fn to_formatter(config_dir: &PathBuf, fmt: config::FmtConfig) -> Result<Formatter> {
    let cmd_path = which(fmt.command.clone())?;
    CLOG.debug(&format!("Found {} at {}", fmt.command, cmd_path.display()));
    let work_dir = match fmt.work_dir {
        Some(work_dir) => config_dir.join(work_dir),
        None => config_dir.clone(),
    };

    return Ok(Formatter {
        command: cmd_path,
        options: fmt.options,
        work_dir: work_dir,
        includes: fmt.includes,
        excludes: fmt.excludes,
    });
}
