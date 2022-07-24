use rhai::plugin::*;

#[export_module]
pub mod matrix_functions {
    use nalgebra::DMatrix;
    use polars::prelude::{CsvReader, DataType, SerReader};
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    #[rhai_fn(name = "inv", return_raw)]
    pub fn invert_matrix(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        // Turn into Vec<Vec<f64>>
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
            if matrix_as_vec[0][0].is::<FLOAT>() {
                matrix_as_vec[i][j].as_float().unwrap()
            } else {
                matrix_as_vec[i][j].as_int().unwrap() as FLOAT
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
            if matrix_as_vec[0][0].is::<FLOAT>() {
                matrix_as_vec[i][j].as_float().unwrap()
            } else {
                matrix_as_vec[i][j].as_int().unwrap() as FLOAT
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

    #[rhai_fn(name = "ndims")]
    pub fn ndims(matrix: Array) -> INT {
        matrix_size(matrix).len() as INT
    }

    #[rhai_fn(name = "numel")]
    pub fn numel(matrix: Array) -> INT {
        flatten(matrix).len() as INT
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

    #[rhai_fn(name = "zeros", return_raw)]
    pub fn zeros_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<i64>() {
            Ok(zeros_double_input(n.as_int().unwrap(), n.as_int().unwrap()))
        } else if n.is::<Array>() {
            let mut m = n.into_array().unwrap();
            if m.len() == 2 {
                Ok(zeros_double_input(
                    m[0].as_int().unwrap(),
                    m[1].as_int().unwrap(),
                ))
            } else if m.len() > 2 {
                let l = m[0].clone();
                m.remove(0);
                Ok(vec![
                    Dynamic::from_array(
                        zeros_single_input(Dynamic::from_array(m)).unwrap()
                    );
                    l.as_int().unwrap() as usize
                ])
            } else {
                Err(EvalAltResult::ErrorMismatchDataType(
                    format!("Input must be INT or Array"),
                    format!(""),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorMismatchDataType(
                format!("Input must be INT or Array"),
                format!(""),
                Position::NONE,
            )
            .into())
        }
    }

    #[rhai_fn(name = "zeros")]
    pub fn zeros_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for i in 0..nx {
            output.push(Dynamic::from_array(vec![Dynamic::FLOAT_ZERO; ny as usize]))
        }
        output
    }

    #[rhai_fn(name = "ones", return_raw)]
    pub fn ones_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<i64>() {
            Ok(ones_double_input(n.as_int().unwrap(), n.as_int().unwrap()))
        } else if n.is::<Array>() {
            let mut m = n.into_array().unwrap();
            if m.len() == 2 {
                Ok(ones_double_input(
                    m[0].as_int().unwrap(),
                    m[1].as_int().unwrap(),
                ))
            } else if m.len() > 2 {
                let l = m[0].clone();
                m.remove(0);
                Ok(vec![
                    Dynamic::from_array(
                        ones_single_input(Dynamic::from_array(m)).unwrap()
                    );
                    l.as_int().unwrap() as usize
                ])
            } else {
                Err(EvalAltResult::ErrorMismatchDataType(
                    format!("Input must be INT or Array"),
                    format!(""),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorMismatchDataType(
                format!("Input must be INT or Array"),
                format!(""),
                Position::NONE,
            )
            .into())
        }
    }

    #[rhai_fn(name = "ones")]
    pub fn ones_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for i in 0..nx {
            output.push(Dynamic::from_array(vec![Dynamic::FLOAT_ONE; ny as usize]))
        }
        output
    }

    #[rhai_fn(name = "rand")]
    pub fn rand_float() -> FLOAT {
        rand::random()
    }

    #[rhai_fn(name = "rand", return_raw)]
    pub fn rand_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<i64>() {
            Ok(rand_double_input(n.as_int().unwrap(), n.as_int().unwrap()))
        } else if n.is::<Array>() {
            let mut m = n.into_array().unwrap();
            if m.len() == 2 {
                Ok(rand_double_input(
                    m[0].as_int().unwrap(),
                    m[1].as_int().unwrap(),
                ))
            } else if m.len() > 2 {
                let l = m[0].clone();
                m.remove(0);
                Ok(vec![
                    Dynamic::from_array(
                        rand_single_input(Dynamic::from_array(m)).unwrap()
                    );
                    l.as_int().unwrap() as usize
                ])
            } else {
                Err(EvalAltResult::ErrorMismatchDataType(
                    format!("Input must be INT or Array"),
                    format!(""),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorMismatchDataType(
                format!("Input must be INT or Array"),
                format!(""),
                Position::NONE,
            )
            .into())
        }
    }

    #[rhai_fn(name = "rand")]
    pub fn rand_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for i in 0..nx {
            let mut row = vec![];
            for j in 0..ny {
                row.push(Dynamic::from_float(rand_float()));
            }
            output.push(Dynamic::from_array(row))
        }
        output
    }

    #[rhai_fn(name = "eye", return_raw)]
    pub fn eye_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<i64>() {
            Ok(eye_double_input(n.as_int().unwrap(), n.as_int().unwrap()))
        } else if n.is::<Array>() {
            let mut m = n.into_array().unwrap();
            if m.len() == 2 {
                Ok(eye_double_input(
                    m[0].as_int().unwrap(),
                    m[1].as_int().unwrap(),
                ))
            } else {
                Err(EvalAltResult::ErrorMismatchDataType(
                    format!("Input must be INT or Array"),
                    format!(""),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorMismatchDataType(
                format!("Input must be INT or Array"),
                format!(""),
                Position::NONE,
            )
            .into())
        }
    }

    #[rhai_fn(name = "eye")]
    pub fn eye_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for i in 0..nx {
            let mut row = vec![];
            for j in 0..ny {
                if i == j {
                    row.push(Dynamic::FLOAT_ONE);
                } else {
                    row.push(Dynamic::FLOAT_ZERO);
                }
            }
            output.push(Dynamic::from_array(row))
        }
        output
    }

    /// Returns the contents of an multidimensional array as a 1-D array.
    #[rhai_fn(name = "flatten")]
    pub fn flatten(matrix: Array) -> Array {
        let mut flat: Vec<Dynamic> = vec![];
        for el in matrix {
            if el.is::<Array>() {
                flat.extend(flatten(el.into_array().unwrap()))
            } else {
                flat.push(el);
            }
        }
        flat
    }

    /// Flip a matrix left-to-right
    #[rhai_fn(name = "fliplr", return_raw)]
    pub fn fliplr(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims(matrix.clone()) > 2 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be flipped - too many dims"),
                Position::NONE,
            )
            .into())
        } else {
            // Turn into Vec<Vec<f64>>
            let matrix_as_vec = matrix
                .clone()
                .into_iter()
                .map(|x| x.into_array().unwrap())
                .collect::<Vec<Array>>();

            let w = matrix_as_vec[0].len();
            let h = matrix_as_vec.len();

            // Turn into Vec<Dynamic>
            let mut out = vec![];
            for idx in 0..h {
                let mut new_row = vec![];
                for jdx in 0..w {
                    new_row.push(matrix_as_vec[idx][w - jdx - 1].clone());
                }
                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        }
    }

    /// Flip a matrix up-down
    #[rhai_fn(name = "flipud", return_raw)]
    pub fn flipud(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims(matrix.clone()) > 2 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be flipped - too many dims"),
                Position::NONE,
            )
            .into())
        } else {
            // Turn into Vec<Vec<f64>>
            let matrix_as_vec = matrix
                .clone()
                .into_iter()
                .map(|x| x.into_array().unwrap())
                .collect::<Vec<Array>>();

            let w = matrix_as_vec[0].len();
            let h = matrix_as_vec.len();

            // Turn into Vec<Dynamic>
            let mut out = vec![];
            for idx in 0..h {
                let mut new_row = vec![];
                for jdx in 0..w {
                    new_row.push(matrix_as_vec[h - idx - 1][jdx].clone());
                }
                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        }
    }

    /// Flip a counterclockwise once
    #[rhai_fn(name = "rot90", return_raw)]
    pub fn rot90_once(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims(matrix.clone()) == 1 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be rotated - not enough dimensions"),
                Position::NONE,
            )
            .into())
        } else if ndims(matrix.clone()) > 2 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be rotated - too many dimensions"),
                Position::NONE,
            )
            .into())
        } else {
            // Turn into Vec<Vec<f64>>
            let matrix_as_vec = matrix
                .clone()
                .into_iter()
                .map(|x| x.into_array().unwrap())
                .collect::<Vec<Array>>();

            let w = matrix_as_vec[0].len();
            let h = matrix_as_vec.len();

            // Turn into Vec<Dynamic>
            let mut out = vec![];
            for idx in 0..w {
                let mut new_row = vec![];
                for jdx in 0..h {
                    new_row.push(matrix_as_vec[jdx][w - idx - 1].clone());
                }

                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        }
    }

    /// Flip a counterclockwise `k` times
    #[rhai_fn(name = "rot90", return_raw)]
    pub fn rot90_ktimes(matrix: Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if k > 1 {
            let new_matrix = rot90_once(matrix);
            match new_matrix {
                Ok(mat) => rot90_ktimes(mat, k - 1),
                Err(e) => Err(e),
            }
        } else {
            rot90_once(matrix)
        }
    }

    #[rhai_fn(name = "mtimes", return_raw)]
    pub fn mtimes(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if matrix_size(matrix1.clone())[1].as_int().unwrap()
            != matrix_size(matrix2.clone())[0].as_int().unwrap()
        {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("The width of the first matrix must be equal to the height of the second matrix"),
                Position::NONE,
            )
                .into());
        }
        // Turn into Vec<Vec<Dynamic>>
        let matrix_as_vec1 = matrix1
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm1 = DMatrix::from_fn(matrix_as_vec1.len(), matrix_as_vec1[0].len(), |i, j| {
            if matrix_as_vec1[0][0].is::<FLOAT>() {
                matrix_as_vec1[i][j].as_float().unwrap()
            } else {
                matrix_as_vec1[i][j].as_int().unwrap() as FLOAT
            }
        });

        // Turn into Vec<Vec<Dynamic>>
        let matrix_as_vec2 = matrix2
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm2 = DMatrix::from_fn(matrix_as_vec2.len(), matrix_as_vec2[0].len(), |i, j| {
            if matrix_as_vec2[0][0].is::<FLOAT>() {
                matrix_as_vec2[i][j].as_float().unwrap()
            } else {
                matrix_as_vec2[i][j].as_int().unwrap() as FLOAT
            }
        });

        // Try to multiple
        let mat = dm1 * dm2;

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

    #[rhai_fn(name = "horzcat", return_raw)]
    pub fn horzcat(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if matrix_size(matrix1.clone())[0].as_int().unwrap()
            != matrix_size(matrix2.clone())[0].as_int().unwrap()
        {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("Matrices must have the same height"),
                Position::NONE,
            )
            .into());
        }

        // Turn into Vec<Vec<Dynamic>>
        let matrix_as_vec1 = matrix1
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm1 = DMatrix::from_fn(matrix_as_vec1.len(), matrix_as_vec1[0].len(), |i, j| {
            if matrix_as_vec1[0][0].is::<FLOAT>() {
                matrix_as_vec1[i][j].as_float().unwrap()
            } else {
                matrix_as_vec1[i][j].as_int().unwrap() as FLOAT
            }
        });

        // Turn into Vec<Vec<Dynamic>>
        let matrix_as_vec2 = matrix2
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm2 = DMatrix::from_fn(matrix_as_vec2.len(), matrix_as_vec2[0].len(), |i, j| {
            if matrix_as_vec2[0][0].is::<FLOAT>() {
                matrix_as_vec2[i][j].as_float().unwrap()
            } else {
                matrix_as_vec2[i][j].as_int().unwrap() as FLOAT
            }
        });

        // Try to multiple
        let w0 = dm1.shape().1;
        let w = dm1.shape().1 + dm2.shape().1;
        let h = dm1.shape().0;
        let mat = DMatrix::from_fn(h, w, |i, j| {
            if j >= w0 {
                dm2[(i, j - w0)]
            } else {
                dm1[(i, j)]
            }
        });

        // Turn into Vec<Dynamic>
        let mut out = vec![];
        for idx in 0..h {
            let mut new_row = vec![];
            for jdx in 0..w {
                new_row.push(Dynamic::from_float(mat[(idx, jdx)]));
            }
            out.push(Dynamic::from_array(new_row));
        }
        Ok(out)
    }

    #[rhai_fn(name = "vertcat", return_raw)]
    pub fn vertcat(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if matrix_size(matrix1.clone())[1].as_int().unwrap()
            != matrix_size(matrix2.clone())[1].as_int().unwrap()
        {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("Matrices must have the same width"),
                Position::NONE,
            )
            .into());
        }

        // Turn into Vec<Vec<Dynamic>>
        let matrix_as_vec1 = matrix1
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm1 = DMatrix::from_fn(matrix_as_vec1.len(), matrix_as_vec1[0].len(), |i, j| {
            if matrix_as_vec1[0][0].is::<FLOAT>() {
                matrix_as_vec1[i][j].as_float().unwrap()
            } else {
                matrix_as_vec1[i][j].as_int().unwrap() as FLOAT
            }
        });

        // Turn into Vec<Vec<Dynamic>>
        let matrix_as_vec2 = matrix2
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        let dm2 = DMatrix::from_fn(matrix_as_vec2.len(), matrix_as_vec2[0].len(), |i, j| {
            if matrix_as_vec2[0][0].is::<FLOAT>() {
                matrix_as_vec2[i][j].as_float().unwrap()
            } else {
                matrix_as_vec2[i][j].as_int().unwrap() as FLOAT
            }
        });

        // Try to multiple
        let h0 = dm1.shape().0;
        let w = dm1.shape().1;
        let h = dm1.shape().0 + dm2.shape().0;
        let mat = DMatrix::from_fn(h, w, |i, j| {
            if i >= h0 {
                dm2[(i - h0, j)]
            } else {
                dm1[(i, j)]
            }
        });

        // Turn into Vec<Dynamic>
        let mut out = vec![];
        for idx in 0..h {
            let mut new_row = vec![];
            for jdx in 0..w {
                new_row.push(Dynamic::from_float(mat[(idx, jdx)]));
            }
            out.push(Dynamic::from_array(new_row));
        }
        Ok(out)
    }

    /// This function can be used in two distinct ways.
    /// 1. If the argument is an 2-D array, `diag` returns an array containing the diagonal of the array.
    /// 2. If the argument is a 1-D array, `diag` returns a matrix containing the argument along the
    /// diagonal and zeros elsewhere.
    #[rhai_fn(name = "diag", return_raw)]
    pub fn diag(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims(matrix.clone()) == 2 {
            // Turn into Vec<Vec<Dynamic>>
            let matrix_as_vec = matrix
                .into_iter()
                .map(|x| x.into_array().unwrap())
                .collect::<Vec<Array>>();

            let mut out = vec![];
            for i in 0..matrix_as_vec.len() {
                out.push(matrix_as_vec[i][i].clone());
            }

            Ok(out)
        } else if ndims(matrix.clone()) == 1 {
            let mut out = vec![];
            for idx in 0..matrix.len() {
                let mut new_row = vec![];
                for jdx in 0..matrix.len() {
                    if idx == jdx {
                        new_row.push(matrix[idx].clone());
                    } else {
                        if matrix[idx].is::<i64>() {
                            new_row.push(Dynamic::ZERO);
                        } else {
                            new_row.push(Dynamic::FLOAT_ZERO);
                        }
                    }
                }
                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        } else {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("Argument must be a 2-D matrix (to extract the diagonal) or a 1-D array (to create a matrix with that diagonal."),
                Position::NONE,
            )
                .into());
        }
    }
}
