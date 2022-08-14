use rhai::plugin::*;

#[export_module]
pub mod ml_functions {
    use crate::{
        array_to_vec_float, if_list_convert_to_vec_float_and_do, if_list_do_int_or_do_float,
    };
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};
    use std::borrow::Borrow;

    #[cfg(feature = "smartcore")]
    use smartcorelib::{
        linalg::naive::dense_matrix::DenseMatrix,
        linear::{
            linear_regression::{LinearRegression, LinearRegressionParameters},
            logistic_regression::{LogisticRegression, LogisticRegressionParameters},
        },
    };

    #[cfg(feature = "smartcore")]
    #[derive(Clone)]
    pub struct Model {
        saved_model: Vec<u8>,
        model_type: String,
    }

    #[cfg(feature = "smartcore")]
    impl Default for Model {
        fn default() -> Self {
            Model {
                saved_model: vec![],
                model_type: String::new(),
            }
        }
    }

    /// Trains a [`smartcore`](https://smartcorelib.org/) machine learning model. The model can then
    /// be used to make predictions with the [`predict`](#predictx-array-model-model---array)
    /// function Available model types are:
    /// 1. `linear` - ordinary least squares linear regression
    /// 2. `logistic` - logistic regression
    /// ```typescript
    /// let xdata = [[1.0, 2.0],
    ///              [2.0, 3.0],
    ///              [3.0, 4.0]];
    /// let ydata = [1.0, 2.0, 3.0];
    /// let model = train(xdata, ydata, "linear");
    /// assert(true);
    /// ```
    #[cfg(feature = "smartcore")]
    #[rhai_fn(name = "train", return_raw, pure)]
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
                match LinearRegression::fit(&Xvec, &yvec, LinearRegressionParameters::default()) {
                    Ok(model) => Ok(Model {
                        saved_model: bincode::serialize(&model).unwrap(),
                        model_type: algorithm_string.to_string(),
                    }),
                    Err(e) => {
                        Err(EvalAltResult::ErrorArithmetic(format!("{e}"), Position::NONE).into())
                    }
                }
            }
            "logistic" => {
                match LogisticRegression::fit(&Xvec, &yvec, LogisticRegressionParameters::default())
                {
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

    /// Uses a [`smartcore`](https://smartcorelib.org/) machine learning model (trained with the [`train`](#trainx-array-y-array-algorithm-immutablestring---model) function to predict dependent variables.
    /// ```typescript
    /// let xdata = [[1.0, 2.0],
    ///              [2.0, 3.0],
    ///              [3.0, 4.0]];
    /// let ydata = [1.0, 2.0, 3.0];
    /// let model = train(xdata, ydata, "linear");
    /// let ypred = predict(xdata, model);
    /// assert(sum(ypred) - sum(ydata) < 0.000001);
    /// ```
    #[cfg(feature = "smartcore")]
    #[rhai_fn(name = "predict", return_raw, pure)]
    pub fn predict_with_model(X: &mut Array, model: Model) -> Result<Array, Box<EvalAltResult>> {
        let Xvec = DenseMatrix::from_2d_vec(
            &X.into_iter()
                .map(|x| array_to_vec_float(&mut x.clone().into_array().unwrap()))
                .collect::<Vec<Vec<FLOAT>>>(),
        );
        let algorithm_string = model.model_type.as_str();
        match algorithm_string {
            "linear" => {
                let model_ready: LinearRegression<FLOAT, DenseMatrix<FLOAT>> =
                    bincode::deserialize(&*model.saved_model).unwrap();
                return match model_ready.predict(&Xvec) {
                    Ok(y) => Ok(y
                        .into_iter()
                        .map(|x| Dynamic::from_float(x))
                        .collect::<Vec<Dynamic>>()),
                    Err(e) => {
                        Err(EvalAltResult::ErrorArithmetic(format!("{e}"), Position::NONE).into())
                    }
                };
            }
            "logistic" => {
                let model_ready: LogisticRegression<FLOAT, DenseMatrix<FLOAT>> =
                    bincode::deserialize(&*model.saved_model).unwrap();
                return match model_ready.predict(&Xvec) {
                    Ok(y) => Ok(y
                        .into_iter()
                        .map(|x| Dynamic::from_float(x))
                        .collect::<Vec<Dynamic>>()),
                    Err(e) => {
                        Err(EvalAltResult::ErrorArithmetic(format!("{e}"), Position::NONE).into())
                    }
                };
            }
            &_ => Err(EvalAltResult::ErrorArithmetic(
                format!("{} is not a recognized model type.", algorithm_string),
                Position::NONE,
            )
            .into()),
        }
    }
}
