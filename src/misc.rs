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

    /// Returns an array of the unique elements in an array.
    /// ```javascript
    /// let data = [1, 2, 2, 2, 5, 4, 4, 2, 5, 8];
    /// let u = unique(data);
    /// assert_eq(u, [1, 2, 4, 5, 8]);
    /// ```
    #[rhai_fn(name = "unique", return_raw)]
    pub fn unique(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        // Convert if needed
        if arr[0].is::<INT>() {
            let mut x = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<INT>>();
            x.sort();
            x.dedup();
            Ok(x.iter().map(|el| Dynamic::from_int(*el)).collect())
        } else if arr[0].is::<FLOAT>() {
            let mut x = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<FLOAT>>();
            x.sort_by(|a, b| a.partial_cmp(b).unwrap());
            x.dedup();
            Ok(x.iter().map(|el| Dynamic::from_float(*el)).collect())
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Elements of array must be either INT or FLOAT"),
                Position::NONE,
            )
            .into())
        }
    }
}
