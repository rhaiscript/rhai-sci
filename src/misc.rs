use rhai::plugin::*;

#[export_module]
pub mod misc_functions {
    use crate::matrix_functions::ndims;
    use nalgebra::DMatrix;
    use polars::prelude::{CsvReader, DataType, SerReader};
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    /// Pretty print arrays.
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

    /// Returns a random number between zero and one.
    /// ```typescript
    /// let r = rand();
    /// assert(r >= 0.0 && r <= 1.0);
    /// ```
    #[rhai_fn(name = "rand")]
    pub fn rand_float() -> FLOAT {
        rand::random()
    }
}
