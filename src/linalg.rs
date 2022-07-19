#[export_module]
mod linalg_functions {
    use nalgebra::DMatrix;
    use rhai::serde::{from_dynamic, to_dynamic};
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT};

    #[rhai_fn(return_raw)]
    pub fn invert_matrix(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        // Turn into Vec<Vec<f64>>
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| from_dynamic(&x).unwrap())
            .collect::<Vec<Vec<f64>>>();

        // Turn into DMatrixs
        let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
            matrix_as_vec[i][j]
        });

        let dm = dm.try_inverse().expect("Cannot invert");

        // Turn into Vec<Dynamic>
        let matrix_as_array = dm.row_iter()
            .map(|x|
                {
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
