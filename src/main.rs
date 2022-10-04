mod toml_format;
mod command_builder;

use std::fs::read_to_string;

use anyhow::{Result, Ok};
use command_builder::run;
use toml_format::Config;

fn main () 
-> Result<()> 
{
	let contents = read_to_string("blackboard.toml")?;
	let toml : Config = toml::from_str(&contents)?;

	// Compiler

	run(
		toml.clone().compile.unwrap_or_default().before.unwrap_or(
			vec![
				String::from("mkdir -p ./target/obj"),
				String::from("mkdir -p ./target/bin"),
			]
		), 
		toml.clone()
	)?;

	run(
		toml.clone().compile.unwrap_or_default().run.unwrap_or(
			vec![ 
				String::from("${compiler} ${compiler_args} -c ${src_files} -I ${header_dirs}"),
			]
		), 
		toml.clone()
	)?;

	run(
		toml.clone().compile.unwrap_or_default().after.unwrap_or(
			vec![ 
				String::from("mv *.o target/obj"),
			]
		), 
		toml.clone()
	)?;

	// Linker

	run(
		toml.clone().linking.unwrap_or_default().before.unwrap_or(
			vec![]
		), 
		toml.clone()
	)?;

	run(
		toml.clone().linking.unwrap_or_default().run.unwrap_or(
			vec![ 
				String::from("${compiler} ${linker_args} -B ${linker} ${obj_files} -o ./target/bin/${name}-${version}"),
			]
		), 
		toml.clone()
	)?;

	run(
		toml.clone().linking.unwrap_or_default().after.unwrap_or(
			vec![ 
				String::from("rm -rf ./target/obj"),
			]
		), 
		toml.clone()
	)?;

	Ok(())
}
