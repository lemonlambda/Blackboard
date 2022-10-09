use crate::toml_format::{Args, Bin, Compiling, Linking, Meta, Tools};

const COMPILER: String = String::from("clang");
const LINKER: String = String::from("clang");

const SRC_FILES: String = String::from("find ./src/ -name \"*.c\"");
const HEADER_DIRS: String = String::from("find ./src/include -type d");
const OBJ_FILES: String = String::from("find ./target/obj -name \"*.o\"");

const COMP_BEFORE: Vec<String> = vec![String::from("mkdir -p ./target/bin ./target/obj")];
const COMP_RUN: Vec<String> = vec![String::from(
    "${compiler} ${compiler_args} -c ${src_files} -I ${header_dirs}",
)];
const COMP_AFTER: Vec<String> = vec![String::from("mv *.o target/obj")];

const LINK_BEFORE: Vec<String> = vec![];
const LINK_RUN: Vec<String> = vec![String::from(
    "${compiler} ${linker_args} -B ${linker} ${obj_files} -o ${output_name}",
)];
const LINK_AFTER: Vec<String> = vec![String::from("rm -rf ./target/obj")];

const COMP_ARGS: String = String::from("-O2");
const LINK_ARGS: String = String::from("");
const OUT_PATH: String = String::from("./target/bin");
const OUT_NAME: String = String::from("${name}-${version}");

impl Default for Bin {
    fn default() -> Self {
        Self {
            tools: Default::default(),
            meta: Default::default(),
            compiling: Default::default(),
            linking: Default::default(),
            args: Default::default(),
            default: Some(false),
            bin_name: Default::default(),
        }
    }
}

impl Default for Tools {
    fn default() -> Self {
        Self {
            compiler: Some(COMPILER),
            linker: Some(LINKER),
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            src_files: Some(SRC_FILES),
            header_dirs: Some(HEADER_DIRS),
            obj_files: Some(OBJ_FILES),
        }
    }
}

impl Default for Compiling {
    fn default() -> Self {
        Self {
            before: Some(COMP_BEFORE),
            run: Some(COMP_RUN),
            after: Some(COMP_AFTER),
        }
    }
}

impl Default for Linking {
    fn default() -> Self {
        Self {
            before: Some(LINK_BEFORE),
            run: Some(LINK_RUN),
            after: Some(LINK_AFTER),
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            compiler_args: Some(COMP_ARGS),
            linker_args: Some(LINK_ARGS),
            out_name: Some(OUT_NAME),
            out_path: Some(OUT_PATH),
        }
    }
}
