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
            self.bin.clone().compiling.before,
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn comp_run(self) -> Result<()> {
        run(
            self.bin.clone().compiling.run,
            self.config,
            self.bin,
            "Run",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn comp_after(self) -> Result<()> {
        run(
            self.bin.clone().compiling.after,
            self.config,
            self.bin,
            "After",
            self.quiet,
        )?;
        Ok(())
    }

    pub fn link_before(self) -> Result<()> {
        run(
            self.bin.clone().linking.before,
            self.config,
            self.bin,
            "Before",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn link_run(self) -> Result<()> {
        run(
            self.bin.clone().linking.run,
            self.config,
            self.bin,
            "Run",
            self.quiet,
        )?;
        Ok(())
    }
    pub fn link_after(self) -> Result<()> {
        run(
            self.bin.clone().linking.after,
            self.config,
            self.bin,
            "After",
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
    let commands = commands
        .into_iter()
        .map(|x| {
            let mut last = x.clone();
            loop {
                #[rustfmt::skip]
            let tmp = last
                .replace("${out_name}", &bin.clone().args.out_name)
                .replace("${compiler}", &bin.clone().tools.compiler)
                .replace("${linker}", &bin.clone().tools.linker)
                .replace("${src_files}", &format!(
                    "$({})", 
                    bin.clone().meta.src_files)
                )
                .replace("${header_dirs}", &format!(
                    "$({})", 
                    bin.clone().meta.header_dirs)
                )
                .replace("${obj_files}", &format!(
                    "$({})", 
                    bin.clone().meta.obj_files)
                )
                .replace("${name}", &config.clone().package.name)
                .replace("${version}", &config.clone().package.version)
                .replace("${compiler_args}", &bin.clone().args.compiler_args)
                .replace("${linker_args}", &bin.clone().args.linker_args)
                .replace("${out_path}", &bin.clone().args.out_path);
                if last == tmp {
                    return tmp;
                }
                last = tmp;
            }
        })
        .collect::<Vec<String>>();

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
