mod command_builder;
mod toml_format;

use std::{env, fs::read_to_string, path::Path};

use anyhow::{Ok, Result};
use clap::{command, Parser, Subcommand};
use colored::Colorize;
use command_builder::run;
use toml_format::Config;

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
        #[arg(short, long, default_value_t = false)]
        quiet: bool,

        #[arg(short, long, default_value = "./")]
        path: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let SubCommand::Build { quiet, path } = args.command {
        let root = Path::new(&path);
        assert!(env::set_current_dir(&root).is_ok());

        let contents = read_to_string("blackboard.toml")?;
        let toml: Config = toml::from_str(&contents)?;

        // Compiler

        if !quiet {
            println!("{}", "Compiling...".bright_magenta().bold());
        }

        run(
            toml.clone()
                .compile
                .unwrap_or_default()
                .before
                .unwrap_or(vec![String::from("mkdir -p ./target/obj ./target/bin")]),
            toml.clone(),
            "Before",
            quiet,
        )?;

        run(
            toml.clone()
                .compile
                .unwrap_or_default()
                .run
                .unwrap_or(vec![String::from(
                    "${compiler} ${compiler_args} -c ${src_files} -I ${header_dirs}",
                )]),
            toml.clone(),
            "Run",
            quiet,
        )?;

        run(
            toml.clone()
                .compile
                .unwrap_or_default()
                .after
                .unwrap_or(vec![String::from("mv *.o target/obj")]),
            toml.clone(),
            "After",
            quiet,
        )?;

        // Linker

        if !quiet {
            println!("{}", "Linking...".bright_magenta().bold());
        }

        run(
            toml.clone()
                .linking
                .unwrap_or_default()
                .before
                .unwrap_or(vec![]),
            toml.clone(),
            "Before",
            quiet,
        )?;

        run(
            toml.clone()
                .linking
                .unwrap_or_default()
                .run
                .unwrap_or(vec![String::from(
                    "${compiler} ${linker_args} -B ${linker} ${obj_files} -o ${output_name}",
                )]),
            toml.clone(),
            "Run",
            quiet,
        )?;

        run(
            toml.clone()
                .linking
                .unwrap_or_default()
                .after
                .unwrap_or(vec![String::from("rm -rf ./target/obj")]),
            toml.clone(),
            "After",
            quiet,
        )?;

        Ok(())
    } else {
        Ok(())
    }
}
