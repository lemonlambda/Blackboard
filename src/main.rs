mod toml_format;

use std::fs::read_to_string;

use anyhow::{Result, Ok};
use toml_format::Config;

fn main () 
-> Result<()> 
{
    let contents = read_to_string("blackboard.toml")?;
    let toml : Config = toml::from_str(&contents)?;

    Ok(())
}
