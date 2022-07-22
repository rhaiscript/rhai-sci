use nalgebra::OMatrix;
use rhai::plugin::*;

#[export_module]
pub mod linalg_functions {
    use nalgebra::DMatrix;
    use rhai::{Array, Dynamic, EvalAltResult, Position};

    #[rhai_fn(name = "inv", return_raw)]
    pub fn invert_matrix(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        // Turn into Vec<Vec<f64>>
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
            if matrix_as_vec[0][0].is::<f64>() {
                matrix_as_vec[i][j].as_float().unwrap()
            } else {
                matrix_as_vec[i][j].as_int().unwrap() as f64
            }
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
                let mut out = vec![];
                for idx in 0..mat.shape().0 {
                    let mut new_row = vec![];
                    for jdx in 0..mat.shape().1 {
                        new_row.push(Dynamic::from_float(mat[(idx, jdx)]));
                    }
                    out.push(Dynamic::from_array(new_row));
                }
                Ok(out)
            }
        }
    }
}
