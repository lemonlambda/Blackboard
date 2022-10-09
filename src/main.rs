mod command_builder;
mod toml_defaults;
mod toml_format;

use std::{env, fs::read_to_string, path::Path};

use anyhow::{Ok, Result};
use clap::{command, Parser, Subcommand};
use colored::Colorize;
use toml_format::{Bin, Config};

use crate::command_builder::CommandBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Test,
    Build {
        #[clap(name = "BUILD_NAME", default_value = "")]
        build_name: String,

        #[arg(short, long, default_value_t = false)]
        quiet: bool,

        #[arg(short, long, default_value = "./")]
        path: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let SubCommand::Build {
        build_name,
        quiet,
        path,
    } = args.command
    {
        let root = Path::new(&path);
        assert!(env::set_current_dir(&root).is_ok());

        let contents = read_to_string("blackboard.toml")?;
        let toml: Config = toml::from_str(&contents)?;

        for x in toml.clone().bin {
            if build_name != "" {
                if x.bin_name == build_name {
                    build(toml, x, quiet)?;
                    break;
                }
            } else {
                if x.default == Some(true) {
                    build(toml, x, quiet)?;
                    break;
                }
            }
        }
    } else {
    }

    Ok(())
}

fn build(toml: Config, x: Bin, quiet: bool) -> Result<()> {
    let builder = CommandBuilder::new(toml.clone(), x, quiet);

    // Compiler

    if !quiet {
        println!("{}", "Compiling...".bright_magenta().bold());
    }
    builder.clone().comp_before()?;
    builder.clone().comp_run()?;
    builder.clone().comp_after()?;

    // Linker

    if !quiet {
        println!("{}", "Linking...".bright_magenta().bold());
    }

    builder.clone().link_before()?;
    builder.clone().link_run()?;
    builder.clone().link_after()?;

    Ok(())
}
