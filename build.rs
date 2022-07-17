use std::io::Write;
use rhai::{packages::Package, plugin::*, ScriptFnMetadata};
use itertools::Itertools;
use rhai_rand::RandomPackage;

fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=scripts");
    println!("cargo:rerun-if-changed=build.rs");

    // Read directory of paths
    let paths = std::fs::read_dir("scripts").unwrap();

    // Open file to write to
    let mut func_file = std::fs::File::create(
        std::env::var("OUT_DIR").unwrap() + "rhai-sci-compiled.txt"
    ).unwrap();

    // Build library and test files
    for path in paths {
        let name = path.unwrap().path();
        if name.clone().to_str().unwrap().ends_with(".rhai") {
            let contents = std::fs::read_to_string(name.clone()).unwrap();
            write!(func_file, "{contents}\n\n").expect("Cannot write to {func_file}");
        }
    }

    // Make a file for documentation
    let mut doc_file = std::fs::File::create("docs/rhai-sci-docs.md").unwrap();

    // Build an engine for doctests
    let mut engine = Engine::new();
    let ast = engine.compile(std::fs::read_to_string(
        std::env::var("OUT_DIR").unwrap() + "rhai-sci-compiled.txt"
    ).unwrap()).unwrap();
    engine.register_global_module(RandomPackage::new().as_shared_module());
    engine.register_global_module(rhai::Shared::new(Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap()));

    // Write functions
    write!(doc_file, "# Functions\n This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.\n").expect("Cannot write to {test_file}");
    let mut indented = false;
    let good_iter: Vec<ScriptFnMetadata> = ast.iter_functions().sorted().collect();
    for idx in 0..good_iter.len() {
        let function = good_iter[idx].clone();
        if function.access == FnAccess::Public && !function.name.starts_with("anon") {
            // Pull out basic info
            let name = function.name;
            let params = function.params.join(", ");
            let comments = function.comments.join("\n").replace("///", "").replace("/**", "").replace("**/", "");

            // Check if there are multiple arities, and if so add a header and indent
            if idx < good_iter.len()-1 {
                if name == good_iter[idx + 1].name && !indented {
                    write!(doc_file, "## `{name}`\n").expect("Cannot write to {doc_file}");
                    indented = true;
                }
            }

            // Print definition with right level of indentation
            if indented {
                write!(doc_file, "### `{name}({params})`\n{comments}\n").expect("Cannot write to {doc_file}");
            } else {
                write!(doc_file, "## `{name}({params})`\n{comments}\n").expect("Cannot write to {doc_file}");
            }

            // End indentation when its time
            if idx != 0 && idx < good_iter.len()-1 {
                if name == good_iter[idx - 1].name && name != good_iter[idx + 1].name {
                    indented = false;
                }
            }

            // Run doc tests
            let code = comments.split("```").collect::<Vec<&str>>();
            for i in (1..code.len()).step_by(2) {
                let clean_code = code[i].replace("javascript", "").replace("\n", "");
                println!("{clean_code}");
                assert!(engine.eval::<bool>(&clean_code).unwrap());
            }
        }
    }

    // Write the constants
    write!(doc_file, "# Constants\n<table><tr><th>Name</th><th>Value</th></tr>").expect("Cannot write to {doc_file}");
    for (name, _, value) in ast.iter_literal_variables(true, false) {
        write!(doc_file, "<tr><td><code>{name}<code></td><td><code>{value}<code></td></tr>").expect("Cannot write to {doc_file}");
    }
    write!(doc_file, "</table>").expect("Cannot write to {doc_file}");

}