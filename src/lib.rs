#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-docs.md"))]
#![doc = include_str!("../docs/highlight.html")]

use rhai::{def_package, packages::Package, plugin::*, Engine, EvalAltResult};
use rhai_rand::RandomPackage;
mod linalg;
use linalg::linalg_functions;

def_package! {
    /// Package for scientific computing
    pub SciPackage(lib) {

        // Load random package
        RandomPackage::init(lib);

        combine_with_exported_module!(lib, "linalg", linalg_functions);

        // Load scripts
        let engine = Engine::new();
        let ast = engine.compile(include_str!(concat!(env!("OUT_DIR"), "/rhai-sci-compiled.txt"))).unwrap();
        let my_module = Module::eval_ast_as_new(rhai::Scope::new(), &ast, &engine).unwrap();
        lib.fill_with(&my_module);
    }
}

/// This provides the ability to easily evaluate a line (or lines) of code without explicitly
/// setting up a script engine
/// ```
/// use rhai_sci::eval;
/// use rhai::FLOAT;
/// print!("{:?}", eval::<FLOAT>("let x = max(5, 2); x + min(3, 72)"));
/// ```
pub fn eval<T: Clone + 'static>(script: &str) -> Result<T, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.eval::<T>(script)
}

use polars::prelude::{
    BooleanChunked, BooleanChunkedBuilder, CsvReader, DataFrame, DataType, NamedFrom, PolarsError,
    SerReader, Series,
};

///
/// ```
/// print!("{:?}", rhai_sci::validate_and_read("https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv"));
/// ```
pub fn validate_and_read<P>(file_path: P) -> Vec<Vec<f64>>
where
    P: AsRef<std::path::Path>,
{
    let file_path_as_str = file_path.as_ref().to_str().unwrap();

    match CsvReader::from_path(file_path_as_str) {
        Ok(csv) => {
            let x = csv
                .infer_schema(Some(10))
                .has_header(
                    csv_sniffer::Sniffer::new()
                        .sniff_path(file_path_as_str.clone())
                        .expect("Cannot sniff file")
                        .dialect
                        .header
                        .has_header_row,
                )
                .finish()
                .expect("Cannot read file as CSV")
                .drop_nulls(None)
                .expect("Cannot remove null values");
            let mut final_output = vec![];
            for series in x.get_columns() {
                let col: Vec<f64> = series
                    .cast(&DataType::Float64)
                    .expect("TODO: panic message")
                    .f64()
                    .unwrap()
                    .into_no_null_iter()
                    .collect();
                final_output.push(col);
            }
            final_output
        }
        Err(_) => {
            if let Ok(_) = url::Url::parse(file_path_as_str) {
                let file_contents = minreq::get(file_path_as_str)
                    .send()
                    .expect("Could not open URL");
                let temp = temp_file::with_contents(file_contents.as_bytes());

                validate_and_read(temp.path().to_str().unwrap())
            } else {
                panic!(
                    "The string {} is not a valid URL or file path.",
                    file_path_as_str
                )
            }
        }
    }
}
