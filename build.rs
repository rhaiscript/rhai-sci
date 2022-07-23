use itertools::Itertools;
use rhai::{packages::Package, plugin::*, Engine, ScriptFnMetadata};
use std::io::Write;

fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=scripts");
    println!("cargo:rerun-if-changed=build.rs");

    // Read directory of paths
    let paths = std::fs::read_dir("scripts").unwrap();

    // Open file to write to
    let mut func_file =
        std::fs::File::create(std::env::var("OUT_DIR").unwrap() + "/rhai-sci-compiled.txt")
            .unwrap();

    // Build library and test files
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

    // Statistics functions
    engine.register_result_fn("max", stats::gen_max);
    engine.register_result_fn("max", stats::array_max);
    engine.register_result_fn("min", stats::gen_min);
    engine.register_result_fn("min", stats::array_min);
    engine.register_result_fn("maxk", stats::maxk);
    engine.register_result_fn("mink", stats::mink);
    engine.register_result_fn("sum", stats::sum);
    engine.register_result_fn("mean", stats::mean);
    engine.register_result_fn("argmin", stats::argmin);
    engine.register_result_fn("argmax", stats::argmax);
    engine.register_result_fn("bounds", stats::bounds);

    // Matrix functions
    engine.register_result_fn("inv", matrix_functions::invert_matrix);
    engine.register_result_fn("read_matrix", matrix_functions::read_matrix);
    engine.register_fn("transpose", matrix_functions::transpose);
    engine.register_fn("size", matrix_functions::matrix_size);
    engine.register_fn("ndims", matrix_functions::ndims);
    engine.register_fn("numel", matrix_functions::numel);
    engine.register_result_fn("zeros", matrix_functions::zeros_single_input);
    engine.register_fn("zeros", matrix_functions::zeros_double_input);
    engine.register_result_fn("ones", matrix_functions::ones_single_input);
    engine.register_fn("ones", matrix_functions::ones_double_input);
    engine.register_fn("rand", matrix_functions::rand_float);
    engine.register_result_fn("rand", matrix_functions::rand_single_input);
    engine.register_fn("rand", matrix_functions::rand_double_input);
    engine.register_result_fn("eye", matrix_functions::eye_single_input);
    engine.register_fn("eye", matrix_functions::eye_double_input);
    engine.register_fn("flatten", matrix_functions::flatten);
    engine.register_result_fn("fliplr", matrix_functions::fliplr);
    engine.register_result_fn("flipud", matrix_functions::flipud);
    engine.register_result_fn("rot90", matrix_functions::rot90_once);
    engine.register_result_fn("rot90", matrix_functions::rot90_ktimes);
    engine.register_result_fn("mtimes", matrix_functions::mtimes);
    engine.register_result_fn("horzcat", matrix_functions::horzcat);
    engine.register_result_fn("vertcat", matrix_functions::vertcat);
    engine.register_result_fn("diag", matrix_functions::diag);

    // Add rand and create engine
    engine.register_global_module(rhai::Shared::new(
        Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap(),
    ));

    // Write functions
    write!(doc_file, "# Functions\n This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.\n").expect("Cannot write to {test_file}");
    let mut indented = false;
    let good_iter: Vec<ScriptFnMetadata> = ast.iter_functions().sorted().collect();
    for idx in 0..good_iter.len() {
        let function = good_iter[idx].clone();
        // if function.access == FnAccess::Public && !function.name.starts_with("anon") {
        // Pull out basic info
        let name = function.name;
        let params = function.params.join(", ");
        let comments = function
            .comments
            .join("\n")
            .replace("///", "")
            .replace("/**", "")
            .replace("**/", "");

        // Check if there are multiple arities, and if so add a header and indent
        if idx < good_iter.len() - 1 {
            if name == good_iter[idx + 1].name && !indented {
                write!(doc_file, "## `{name}`\n").expect("Cannot write to {doc_file}");
                indented = true;
            }
        }

        // Print definition with right level of indentation
        if indented {
            write!(doc_file, "### `{name}({params})`\n{comments}\n")
                .expect("Cannot write to {doc_file}");
        } else {
            write!(doc_file, "## `{name}({params})`\n{comments}\n")
                .expect("Cannot write to {doc_file}");
        }

        // End indentation when its time
        if idx != 0 && idx < good_iter.len() - 1 {
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
        // }
    }
}

include!("src/matrix.rs");
include!("src/basic_statistics.rs");
include!("src/utils.rs");
