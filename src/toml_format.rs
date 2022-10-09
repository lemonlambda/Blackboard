use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub package: Package,
    pub bins: Vec<Bin>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Bin {
    pub tools: Option<Tools>,
    pub meta: Option<Meta>,
    pub compiling: Option<Compiling>,
    pub linking: Option<Linking>,
    pub args: Option<Args>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Tools {
    pub compiler: Option<String>,
    pub linker: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    pub src_files: Option<String>,
    pub header_dirs: Option<String>,
    pub obj_files: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Args {
    pub compiler_args: Option<String>,
    pub linker_args: Option<String>,
    pub out_path: Option<String>,
    pub output_name: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Compiling {
    pub before: Option<Vec<String>>,
    pub run: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Linking {
    pub before: Option<Vec<String>>,
    pub run: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
}
