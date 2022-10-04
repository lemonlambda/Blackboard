use std::process::Command;

use crate::toml_format::Config;

use anyhow::{Result, Ok};

pub
fn run (commands : Vec<String>, config : Config)
-> Result<()>
{
	let commands = commands.into_iter().map(|x| {
		x
		.replace("${compiler}", &config.clone().tools.compiler.unwrap_or(String::from("clang")))
		.replace("${linker}", &config.clone().tools.linker.unwrap_or(String::from("clang")))
		.replace("${src_files}", &config.clone().meta.src_files.unwrap_or(String::from("find ./src/* -name \"*.c\"")))
		.replace("${header_dirs}", &config.clone().meta.header_dirs.unwrap_or(String::from("find ./src/include/* -type d")))
		.replace("${obj_files}", &config.clone().meta.obj_files.unwrap_or(String::from("find ./target/obj/* -name \"*.o\"")))
		.replace("${name}", &config.clone().package.name)
		.replace("${version}", &config.clone().package.version)
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
		Command::new(cmd)
			.args([c, &x])
			.output()?;
	}

	Ok(())
}
