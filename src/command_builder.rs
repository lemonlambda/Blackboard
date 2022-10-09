use std::process::Command;

use crate::toml_format::{Bin, Config};

use anyhow::{Ok, Result};
use colored::Colorize;

#[derive(Clone)]
pub struct CommandBuilder {
    config: Config,
    bin: Bin,
    quiet: bool,
}

impl CommandBuilder {
    pub fn new(config: Config, bin: Bin, quiet: bool) -> Self {
        Self { config, bin, quiet }
    }

    pub fn comp_before(self) -> Result<()> {
        run(
            self.bin
                .clone()
                .compiling
                .unwrap_or_default()
                .before
                .unwrap_or_default(),
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn comp_run(self) -> Result<()> {
        run(
            self.bin
                .clone()
                .compiling
                .unwrap_or_default()
                .run
                .unwrap_or_default(),
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn comp_after(self) -> Result<()> {
        run(
            self.bin
                .clone()
                .compiling
                .unwrap_or_default()
                .after
                .unwrap_or_default(),
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }

    pub fn link_before(self) -> Result<()> {
        run(
            self.bin
                .clone()
                .linking
                .unwrap_or_default()
                .before
                .unwrap_or_default(),
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn link_run(self) -> Result<()> {
        run(
            self.bin
                .clone()
                .linking
                .unwrap_or_default()
                .run
                .unwrap_or_default(),
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn link_after(self) -> Result<()> {
        run(
            self.bin
                .clone()
                .linking
                .unwrap_or_default()
                .after
                .unwrap_or_default(),
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
}

pub fn run(
    commands: Vec<String>,
    config: Config,
    bin: Bin,
    stage: &str,
    quiet: bool,
) -> Result<()> {
    let commands = commands.into_iter().map(|x| {
        let mut last = x.clone();
        loop {
            #[rustfmt::skip]
            let tmp = last
                .replace("${out_name}", &bin.clone().args.unwrap_or_default().out_name.unwrap_or_default())
                .replace("${compiler}", &bin.clone().tools.unwrap_or_default().compiler.unwrap_or_default())
                .replace("${linker}", &bin.clone().tools.unwrap_or_default().linker.unwrap_or_default())
                .replace("${src_files}", &format!(
                    "$({})", 
                    bin.clone().meta.unwrap_or_default().src_files.unwrap_or_default())
                )
                .replace("${header_dirs}", &format!(
                    "$({})", 
                    bin.clone().meta.unwrap_or_default().header_dirs.unwrap_or_default())
                )
                .replace("${obj_files}", &format!(
                    "$({})", 
                    bin.clone().meta.unwrap_or_default().obj_files.unwrap_or_default())
                )
                .replace("${name}", &config.clone().package.name)
                .replace("${version}", &config.clone().package.version)
                .replace("${compiler_args}", &bin.clone().args.unwrap_or_default().compiler_args.unwrap_or_default())
                .replace("${linker_args}", &bin.clone().args.unwrap_or_default().linker_args.unwrap_or_default())
                .replace("${out_path}", &bin.clone().args.unwrap_or_default().out_path.unwrap_or_default());
            if last == tmp {
                return tmp;
            }
            last = tmp;
        }
	}).collect::<Vec<String>>();

    let cmd;
    let c;

    if cfg!(windows) {
        cmd = "cmd";
        c = "/C";
    } else {
        cmd = "sh";
        c = "-c";
    }

    for x in commands {
        if !quiet {
            println!("\t{} {}: {x}", stage.bright_blue(), "Task".bright_black());
        }
        let output = Command::new(cmd).args([c, &x]).output()?;
        if !String::from_utf8_lossy(&output.stdout).is_empty() {
            if quiet {
                println!("\t{} {}: {x}", stage.bright_blue(), "Task".bright_black());
            }
            print!(
                "\t{} {}{}{}: {}",
                "Stdout".yellow(),
                "[".bright_black(),
                output.status.code().unwrap_or(0),
                "]".bright_black(),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        if !String::from_utf8_lossy(&output.stderr).is_empty() {
            if quiet {
                println!("\t{} {}: {x}", stage.bright_blue(), "Task".bright_black());
            }
            print!(
                "\t{} {}{}{}: {}",
                "Stderr".red(),
                "[".bright_black(),
                output.status.code().unwrap_or(0),
                "]".bright_black(),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    Ok(())
}
