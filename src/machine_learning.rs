use rhai::plugin::*;

#[export_module]
pub mod ml_functions {
    use crate::{
        array_to_vec_float, if_list_convert_to_vec_float_and_do, if_list_do_int_or_do_float,
    };
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};
    use std::borrow::Borrow;

    #[derive(Clone)]
    pub struct Model {
        saved_model: Vec<u8>,
        model_type: String,
    }

    impl Default for Model {
        fn default() -> Self {
            Model {
                saved_model: vec![],
                model_type: String::new(),
            }
        }
    }

    /// Returns an array of the unique elements in an array.
    /// ```typescript
    /// let xdata = [[1.0, 2.0],
    ///              [2.0, 3.0],
    ///              [3.0, 4.0]];
    /// let ydata = [1.0, 2.0, 3.0];
    /// let model = train_model(xdata, ydata, "linear");
    /// assert(true);
    /// ```
    #[rhai_fn(name = "train_model", return_raw, pure)]
    pub fn train_model(
        X: &mut Array,
        y: Array,
        algorithm: ImmutableString,
    ) -> Result<Model, Box<EvalAltResult>> {
        let algorithm_string = algorithm.as_str();
        let yvec = array_to_vec_float(&mut y.clone());
        let Xvec = smartcorelib::linalg::naive::dense_matrix::DenseMatrix::from_2d_vec(
            &X.into_iter()
                .map(|x| array_to_vec_float(&mut x.clone().into_array().unwrap()))
                .collect::<Vec<Vec<FLOAT>>>(),
        );
        match algorithm_string {
            "linear" => {
                match smartcorelib::linear::linear_regression::LinearRegression::fit(
                    &Xvec,
                    &yvec,
                    smartcorelib::linear::linear_regression::LinearRegressionParameters::default(),
                ) {
                    Ok(model) => Ok(Model {
                        saved_model: bincode::serialize(&model).unwrap(),
                        model_type: algorithm_string.to_string(),
                    }),
                    Err(e) => {
                        Err(EvalAltResult::ErrorArithmetic(format!("{e}"), Position::NONE).into())
                    }
                }
            }
            &_ => Err(EvalAltResult::ErrorArithmetic(
                format!("{} is not a recognized model type.", algorithm_string),
                Position::NONE,
            )
            .into()),
        }
    }
}
