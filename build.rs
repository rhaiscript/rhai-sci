use std::io::Write;
use rhai::{ def_package, packages::Package, plugin::*, EvalAltResult};
use itertools::Itertools;

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
    let engine = Engine::new();
    let ast = engine.compile(include_str!("scripts/rhai-lab-compiled.txt")).unwrap();
    write!(doc_file, "# Functions\n").expect("Cannot write to {test_file}");
    for function in ast.iter_functions().sorted() {
        if function.access == FnAccess::Public && !function.name.starts_with("anon") {
            let name = function.name;
            let params = function.params.join(", ");
            let comments = function.comments.join("\n").replace("///", "").replace("/**", "").replace("**/", "");
            write!(doc_file, "## {name}({params})\n{comments}\n").expect("Cannot write to {test_file}");
        }
    }

    // Write the constants
    write!(doc_file, "# Constants\n<table><tr><th>Name</th><th>Value</th></tr>").expect("Cannot write to {doc_file}");
    for (name, _, value) in ast.iter_literal_variables(true, false) {
        write!(doc_file, "<tr><td>{name}</td><td>{value}</td></tr>").expect("Cannot write to {doc_file}");
    }
    write!(doc_file, "</table>").expect("Cannot write to {doc_file}");

}