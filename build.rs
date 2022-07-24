use itertools::Itertools;
use rhai::{packages::Package, plugin::*, Engine, ScriptFnMetadata};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=scripts");
    println!("cargo:rerun-if-changed=src"); // TODO: Remove this block once functions are no longer in Rhai
    println!("cargo:rerun-if-changed=build.rs");

    // Read directory of paths - TODO: Remove this block once functions are no longer in Rhai
    let paths = std::fs::read_dir("scripts").unwrap();

    // Open file to write to - TODO: Remove this block once functions are no longer in Rhai
    let mut func_file =
        std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/rhai-sci-compiled.txt")
            .unwrap();

    // Build library and test files - TODO: Remove this block once functions are no longer in Rhai
    for path in paths {
        let name = path.unwrap().path();
        if name.clone().to_str().unwrap().ends_with(".rhai") {
            let contents = std::fs::read_to_string(name.clone()).unwrap();
            write!(func_file, "{contents}\n\n").expect("Cannot write to {func_file}");
        }
    }

    // Make a file for documentation
    let mut doc_file =
        std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/rhai-sci-docs.md").unwrap();

    // Build an engine for doctests
    let mut engine = Engine::new();
    let ast = engine
        .compile(
            std::fs::read_to_string(std::env::var("OUT_DIR").unwrap() + "/rhai-sci-compiled.txt")
                .unwrap(),
        )
        .unwrap();

    // Add rand and create engine
    engine.register_global_module(rhai::Shared::new(
        Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap(),
    ));

    // Add custom functions from Rust
    let mut lib = Module::new();
    combine_with_exported_module!(&mut lib, "rhai_sci_matrix_function", matrix_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_utility_functions", util_functions);
    combine_with_exported_module!(&mut lib, "rhai_sci_basic_stats", stats);
    engine.register_global_module(rhai::Shared::new(lib));

    // Extract metadata
    let mut json_fns = engine.gen_fn_metadata_to_json(false).unwrap();
    println!("{json_fns}");
    let v: HashMap<String, Vec<Function>> = serde_json::from_str(&json_fns).unwrap();
    for function in v["functions"].clone() {
        println!("{:?}", function);
    }

    let function_list = v["functions"].clone();

    // Write functions
    write!(doc_file, "# Functions\n This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.\n").expect("Cannot write to {test_file}");
    let mut indented = false;
    for (idx, function) in function_list.iter().enumerate() {
        let mut function = function.clone();
        // Pull out basic info
        let name = function.name;
        if !name.starts_with("anon") {
            // let params = function.params.join(", ");

            let comments = match function.docComments {
                None => "".to_owned(),
                Some(strings) => strings.join("\n"),
            }
            .replace("///", "")
            .replace("/**", "")
            .replace("**/", "");

            let signature = function
                .signature
                .replace("core::result::", "")
                .replace("rhai::types::dynamic::", "")
                .replace("types::dynamic::", "")
                .replace("alloc::boxed::", "")
                .replace("alloc::vec::", "")
                .replace("rhai::types::error::", "");

            // Check if there are multiple arities, and if so add a header and indent
            if idx < function_list.len() - 1 {
                if name == function_list[idx + 1].name && !indented {
                    write!(doc_file, "## `{name}`\n").expect("Cannot write to {doc_file}");
                    indented = true;
                }
            }

            // Print definition with right level of indentation
            if indented {
                write!(doc_file, "### `{signature}`\n{comments}\n")
                    .expect("Cannot write to {doc_file}");
            } else {
                write!(doc_file, "## `{signature}`\n{comments}\n")
                    .expect("Cannot write to {doc_file}");
            }

            // End indentation when its time
            if idx != 0 && idx < function_list.len() - 1 {
                if name == function_list[idx - 1].name && name != function_list[idx + 1].name {
                    indented = false;
                }
            }

            // Run doc tests
            let code = comments.split("```").collect::<Vec<&str>>();
            for i in (1..code.len()).step_by(2) {
                let clean_code = code[i]
                    .replace("javascript", "")
                    .replace("typescript", "")
                    .replace("rhai", "");
                println!("{clean_code}");
                assert!(engine.eval::<bool>(&clean_code).unwrap());
            }
        }
    }
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Function {
    pub access: String,
    pub baseHash: u128,
    pub fullHash: u128,
    pub name: String,
    pub namespace: String,
    pub numParams: usize,
    pub params: Option<Vec<HashMap<String, String>>>,
    pub signature: String,
    pub returnType: Option<String>,
    pub docComments: Option<Vec<String>>,
}

include!("src/matrix.rs");
include!("src/basic_statistics.rs");
include!("src/utils.rs");
