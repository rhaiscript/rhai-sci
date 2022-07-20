use rhai::plugin::*;

#[export_module]
pub mod linalg_functions {
    use nalgebra::DMatrix;
    use rhai::serde::{from_dynamic, to_dynamic};
    use rhai::{Array, Dynamic, EvalAltResult, Position};

    #[rhai_fn(return_raw)]
    pub fn invert_matrix(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        // Turn into Vec<Vec<f64>>
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| from_dynamic(&x).unwrap())
            .collect::<Vec<Vec<f64>>>();

        // Turn into DMatrix
        let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
            matrix_as_vec[i][j]
        });

        // Try ot invert
        let dm = dm.try_inverse();

        match dm {
            None => Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be inverted"),
                Position::NONE,
            )
            .into()),

            Some(mat) => {
                // Turn into Vec<Dynamic>
                let matrix_as_array = mat
                    .row_iter()
                    .map(|x| {
                        let mut y = vec![];
                        for el in &x {
                            y.push(*el as f64);
                        }
                        to_dynamic(&y).unwrap()
                    })
                    .collect::<Vec<Dynamic>>();
                Ok(matrix_as_array)
            }
        }
    }
}
