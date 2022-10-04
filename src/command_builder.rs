use std::process::Command;

use crate::toml_format::Config;

use anyhow::{Result, Ok};
use colored::Colorize;

pub
fn run (commands : Vec<String>, config : Config)
-> Result<()>
{
	let commands = commands.into_iter().map(|x| {
		x
		.replace("${output_name}", &config.clone().meta.unwrap_or_default().linker_args.unwrap_or(String::from("${name}-${version}")))
		.replace("${compiler}", &config.clone().tools.unwrap_or_default().compiler.unwrap_or(String::from("clang")))
		.replace("${linker}", &config.clone().tools.unwrap_or_default().linker.unwrap_or(String::from("clang")))
		.replace("${src_files}", &format!(
			"$({})", 
			config.clone().meta.unwrap_or_default().src_files.unwrap_or(String::from("find ./src/ -name *.c"))
		))
		.replace("${header_dirs}", &format!(
			"$({})", 
			config.clone().meta.unwrap_or_default().header_dirs.unwrap_or(String::from("find ./src/include/ -type d"))
		))
		.replace("${obj_files}", &format!(
			"$({})", 
			config.clone().meta.unwrap_or_default().obj_files.unwrap_or(String::from("find ./target/obj/ -name *.o"))
		))
		.replace("${name}", &config.clone().package.name)
		.replace("${version}", &config.clone().package.version)
		.replace("${compiler_args}", &config.clone().meta.unwrap_or_default().compiler_args.unwrap_or(String::from("")))
		.replace("${linker_args}", &config.clone().meta.unwrap_or_default().linker_args.unwrap_or(String::from("")))
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
		println!("{}: {x}", "Running".green());
		let output = Command::new(cmd)
			.args([c, &x])
			.output()?;
		if !String::from_utf8_lossy(&output.stdout).is_empty() {
			println!("{} {}{}{}: {}", "Stdout".yellow(), "[".bright_black(), output.status.code().unwrap_or(0), "]".bright_black(), String::from_utf8_lossy(&output.stdout));
		}
		if !String::from_utf8_lossy(&output.stderr).is_empty() {
			println!("{} {}{}{}: {}", "Stderr".red(), "[".bright_black(), output.status.code().unwrap_or(0), "]".bright_black(), String::from_utf8_lossy(&output.stderr));
		}
	}

	Ok(())
}
