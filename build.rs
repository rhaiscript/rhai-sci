use std::io::Write;
use rhai::{Func, packages::Package, plugin::*};
use itertools::Itertools;
use rhai_rand::RandomPackage;

fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=scripts");
    println!("cargo:rerun-if-changed=build.rs");

    // Read directory of paths
    let paths = std::fs::read_dir("scripts").unwrap();

    // Open file to write to
    let mut func_file = std::fs::File::create("scripts/rhai-lab-compiled.txt").unwrap();
    let mut test_file = std::fs::File::create("tests/rhai-lab-test.txt").unwrap();

    // Build library and test files
    for path in paths {
        let name = path.unwrap().path();
        if name.clone().to_str().unwrap().ends_with(".rhai") {
            let contents = std::fs::read_to_string(name.clone()).unwrap();
            write!(func_file, "{contents}\n\n").expect("Cannot write to {func_file}");
        }
        if name.clone().to_str().unwrap().ends_with(".rhai-test") {
            let contents = std::fs::read_to_string(name.clone()).unwrap();
            write!(test_file, "{contents}\n\n").expect("Cannot write to {test_file}");
        }
    }

    // Build documentation
    let mut doc_file = std::fs::File::create("docs/rhai-lab-docs.md").unwrap();
    let mut engine = Engine::new();
    let ast = engine.compile(std::fs::read_to_string("scripts/rhai-lab-compiled.txt").unwrap()).unwrap();
    engine.register_global_module(RandomPackage::new().as_shared_module());
    engine.register_global_module(rhai::Shared::new(Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap()));
    write!(doc_file, "# Functions\n This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.\n").expect("Cannot write to {test_file}");
    for function in ast.iter_functions().sorted() {
        if function.access == FnAccess::Public && !function.name.starts_with("anon") {
            let name = function.name;
            let params = function.params.join(", ");
            let comments = function.comments.join("\n").replace("///", "").replace("/**", "").replace("**/", "");
            write!(doc_file, "## {name}({params})\n{comments}\n").expect("Cannot write to {test_file}");
            let code = comments.split("```").collect::<Vec<&str>>();
            if code.len() == 3 {
                let clean_code = code[1].replace("rhai", "").replace("\n", "");
                assert!(engine.eval::<bool>(&clean_code).unwrap());
            }
        }
    }

    // Write the constants
    write!(doc_file, "# Constants\n<table><tr><th>Name</th><th>Value</th></tr>").expect("Cannot write to {doc_file}");
    for (name, _, value) in ast.iter_literal_variables(true, false) {
        write!(doc_file, "<tr><td>{name}</td><td>{value}</td></tr>").expect("Cannot write to {doc_file}");
    }
    write!(doc_file, "</table>").expect("Cannot write to {doc_file}");

}