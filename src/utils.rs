use rhai::plugin::*;

#[export_module]
pub mod util_functions {
    use crate::matrix_functions::ndims;
    use nalgebra::DMatrix;
    use polars::prelude::{CsvReader, DataType, SerReader};
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    #[rhai_fn(name = "pprint")]
    pub fn pprint(matrix: Array) {
        if ndims(matrix.clone()) > 1 {
            for row in matrix {
                println!("{:?}", row);
            }
        } else {
            println!("{:?}", matrix)
        }
    }
}
