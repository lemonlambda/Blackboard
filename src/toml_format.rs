use serde_derive::Deserialize;

use crate::toml_defaults::*;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub package: Package,
    pub bin: Vec<Bin>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Bin {
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub bin_name: String,

    #[serde(default)]
    pub tools: Tools,
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub compiling: Compiling,
    #[serde(default)]
    pub linking: Linking,
    #[serde(default)]
    pub args: Args,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Tools {
    #[serde(default = "COMPILER_fn")]
    pub compiler: String,
    #[serde(default = "LINKER_fn")]
    pub linker: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    #[serde(default = "SRC_FILES_fn")]
    pub src_files: String,
    #[serde(default = "HEADER_DIRS_fn")]
    pub header_dirs: String,
    #[serde(default = "OBJ_FILES_fn")]
    pub obj_files: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Args {
    #[serde(default = "COMP_ARGS_fn")]
    pub compiler_args: String,
    #[serde(default = "LINK_ARGS_fn")]
    pub linker_args: String,
    #[serde(default = "OUT_PATH_fn")]
    pub out_path: String,
    #[serde(default = "OUT_NAME_fn")]
    pub out_name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Compiling {
    #[serde(default = "COMP_BEFORE_fn")]
    pub before: Vec<String>,
    #[serde(default = "COMP_RUN_fn")]
    pub run: Vec<String>,
    #[serde(default = "COMP_AFTER_fn")]
    pub after: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Linking {
    #[serde(default = "LINK_BEFORE_fn")]
    pub before: Vec<String>,
    #[serde(default = "LINK_RUN_fn")]
    pub run: Vec<String>,
    #[serde(default = "LINK_AFTER_fn")]
    pub after: Vec<String>,
}
