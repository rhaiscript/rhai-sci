use rhai::plugin::*;

#[export_module]
pub mod matrix_functions {
    use nalgebra::DMatrix;
    use polars::prelude::{CsvReader, DataType, SerReader};
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, INT};

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

    fn transpose_internal<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    #[rhai_fn(name = "transpose")]
    pub fn transpose(matrix: Array) -> Array {
        let new_matrix = if !matrix[0].is::<Array>() {
            vec![Dynamic::from_array(matrix.clone())]
        } else {
            matrix.clone()
        };
        // Turn into Vec<Vec<f64>>
        let matrix_as_vec = new_matrix
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let mat = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
            if matrix_as_vec[0][0].is::<f64>() {
                matrix_as_vec[i][j].as_float().unwrap()
            } else {
                matrix_as_vec[i][j].as_int().unwrap() as f64
            }
        })
        .transpose();

        // Turn into Vec<Dynamic>
        let mut out = vec![];
        for idx in 0..mat.shape().0 {
            let mut new_row = vec![];
            for jdx in 0..mat.shape().1 {
                new_row.push(Dynamic::from_float(mat[(idx, jdx)]));
            }
            out.push(Dynamic::from_array(new_row));
        }
        out
    }

    #[rhai_fn(name = "size")]
    pub fn matrix_size(matrix: Array) -> Array {
        let mut new_matrix = matrix.clone();

        let mut shape = vec![Dynamic::from_int(new_matrix.len() as INT)];
        loop {
            if new_matrix[0].is::<Array>() {
                new_matrix = new_matrix[0].clone().into_array().unwrap();
                shape.push(Dynamic::from_int(new_matrix.len() as INT));
            } else {
                break;
            }
        }

        shape
    }

    #[rhai_fn(name = "read_matrix", return_raw)]
    pub fn read_matrix(file_path: ImmutableString) -> Result<Array, Box<EvalAltResult>> {
        let file_path_as_str = file_path.as_str();

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

                // Convert into vec of vec
                let mut final_output = vec![];
                for series in x.get_columns() {
                    let col: Vec<f64> = series
                        .cast(&DataType::Float64)
                        .expect("Cannot cast to f64")
                        .f64()
                        .unwrap()
                        .into_no_null_iter()
                        .collect();
                    final_output.push(col);
                }

                final_output = transpose_internal(final_output);

                let matrix_as_array = final_output
                    .into_iter()
                    .map(|x| {
                        let mut y = vec![];
                        for el in &x {
                            y.push(Dynamic::from_float(*el));
                        }
                        Dynamic::from_array(y)
                    })
                    .collect::<Vec<Dynamic>>();

                Ok(matrix_as_array)
            }
            Err(_) => {
                if let Ok(_) = url::Url::parse(file_path_as_str) {
                    let file_contents = minreq::get(file_path_as_str)
                        .send()
                        .expect("Could not open URL");
                    let temp = temp_file::with_contents(file_contents.as_bytes());

                    let temp_file_name: ImmutableString = temp.path().to_str().unwrap().into();
                    read_matrix(temp_file_name)
                } else {
                    panic!(
                        "The string {} is not a valid URL or file path.",
                        file_path_as_str
                    )
                }
            }
        }
    }
}
