use crate::toml_format::{Args, Bin, Compiling, Linking, Meta, Tools};

const COMPILER: &'static str = "clang";
const LINKER: &'static str = "clang";

const SRC_FILES: &'static str = "find ./src/ -name \"*.c\"";
const HEADER_DIRS: &'static str = "find ./src/include -type d";
const OBJ_FILES: &'static str = "find ./target/obj -name \"*.o\"";

const COMP_BEFORE: [&'static str; 1] = ["mkdir -p ./target/bin ./target/obj"];
const COMP_RUN: [&'static str; 1] =
    ["${compiler} ${compiler_args} -c ${src_files} -I ${header_dirs}"];
const COMP_AFTER: [&'static str; 1] = ["mv *.o target/obj"];

const LINK_BEFORE: [&'static str; 0] = [];
const LINK_RUN: [&'static str; 1] =
    ["${compiler} ${linker_args} -B ${linker} ${obj_files} -o ${out_path}/${out_name}"];
const LINK_AFTER: [&'static str; 0] = [];

const COMP_ARGS: &'static str = "-O2";
const LINK_ARGS: &'static str = "";
const OUT_PATH: &'static str = "./target/bin";
const OUT_NAME: &'static str = "${name}-${version}";

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
            compiler: Some(COMPILER.to_string()),
            linker: Some(LINKER.to_string()),
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            src_files: Some(SRC_FILES.to_string()),
            header_dirs: Some(HEADER_DIRS.to_string()),
            obj_files: Some(OBJ_FILES.to_string()),
        }
    }
}

impl Default for Compiling {
    fn default() -> Self {
        Self {
            before: Some(
                COMP_BEFORE
                    .to_vec()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
            run: Some(
                COMP_RUN
                    .to_vec()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
            after: Some(
                COMP_AFTER
                    .to_vec()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
        }
    }
}

impl Default for Linking {
    fn default() -> Self {
        Self {
            before: Some(
                LINK_BEFORE
                    .to_vec()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
            run: Some(
                LINK_RUN
                    .to_vec()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
            after: Some(
                LINK_AFTER
                    .to_vec()
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
            ),
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            compiler_args: Some(COMP_ARGS.to_string()),
            linker_args: Some(LINK_ARGS.to_string()),
            out_name: Some(OUT_NAME.to_string()),
            out_path: Some(OUT_PATH.to_string()),
        }
    }
}
