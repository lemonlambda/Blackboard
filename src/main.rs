mod command_builder;
mod toml_format;

use std::fs::read_to_string;

use anyhow::{Ok, Result};
use colored::Colorize;
use command_builder::run;
use toml_format::Config;

fn main() -> Result<()> {
    let contents = read_to_string("blackboard.toml")?;
    let toml: Config = toml::from_str(&contents)?;

    // Compiler

    println!("{}", "Compiling...".bright_magenta().bold());

    run(
        toml.clone()
            .compile
            .unwrap_or_default()
            .before
            .unwrap_or(vec![
                String::from("mkdir -p ./target/obj"),
                String::from("mkdir -p ./target/bin"),
            ]),
        toml.clone(),
        "Before",
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
    )?;

    run(
        toml.clone()
            .compile
            .unwrap_or_default()
            .after
            .unwrap_or(vec![String::from("mv *.o target/obj")]),
        toml.clone(),
        "After",
    )?;

    // Linker

    println!("{}", "Linking...".bright_magenta().bold());

    run(
        toml.clone()
            .linking
            .unwrap_or_default()
            .before
            .unwrap_or(vec![]),
        toml.clone(),
        "Before",
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
    )?;

    run(
        toml.clone()
            .linking
            .unwrap_or_default()
            .after
            .unwrap_or(vec![String::from("rm -rf ./target/obj")]),
        toml.clone(),
        "After",
    )?;

    Ok(())
}
