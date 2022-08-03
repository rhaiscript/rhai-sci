use rhai::plugin::*;

#[export_module]
pub mod matrix_functions {
    use crate::misc_functions::rand_float;
    use nalgebra::DMatrix;
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Map, Position, FLOAT, INT};
    use std::collections::BTreeMap;

    /// Calculates the inverse of a matrix. Fails if the matrix if not invertible, or if the
    /// elements of the matrix aren't FLOAT or INT.
    /// ```typescript
    /// let x = [[ 1.0,  0.0,  2.0],
    ///          [-1.0,  5.0,  0.0],
    ///          [ 0.0,  3.0, -9.0]];
    /// let x_inverted = inv(x);
    /// assert_eq(x_inverted, [[0.8823529411764706,  -0.11764705882352941,   0.19607843137254902],
    ///                        [0.17647058823529413,  0.17647058823529413,   0.0392156862745098 ],
    ///                        [0.058823529411764705, 0.058823529411764705, -0.09803921568627451]]
    /// );
    /// ```
    /// ```typescript
    /// let x = [[1, 2],
    ///          [3, 4]];
    /// let x_inverted = inv(x);
    /// assert_eq(x_inverted, [[-2.0, 1.0],
    ///                        [1.5, -0.5]]
    /// );
    /// ```
    #[rhai_fn(name = "inv", return_raw, pure)]
    pub fn invert_matrix(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        // Turn into Vec<Vec<FLOAT>>
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| x.clone().into_array().unwrap())
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

    /// Transposes a matrix.
    /// ```typescript
    /// let row = [1, 2, 3, 4];
    /// let column = transpose(row);
    /// assert_eq(column, [[1],
    ///                    [2],
    ///                    [3],
    ///                    [4]]);
    /// ```
    /// ```typescript
    /// let matrix = transpose(eye(3));
    /// assert_eq(matrix, eye(3));
    /// ```
    #[rhai_fn(name = "transpose")]
    pub fn transpose(matrix: Array) -> Array {
        let new_matrix = if !matrix[0].is::<Array>() {
            vec![Dynamic::from_array(matrix.clone())]
        } else {
            matrix.clone()
        };
        // Turn into Vec<Vec<FLOAT>>
        let matrix_as_vec = new_matrix
            .into_iter()
            .map(|x| x.into_array().unwrap())
            .collect::<Vec<Array>>();

        // Turn into Vec<Dynamic>
        let mut out = vec![];
        for idx in 0..matrix_as_vec[0].len() {
            let mut new_row = vec![];
            for jdx in 0..matrix_as_vec.len() {
                new_row.push(matrix_as_vec[jdx][idx].clone());
            }
            out.push(Dynamic::from_array(new_row));
        }
        out
    }

    /// Returns an array indicating the size of the matrix along each dimension, passed by reference.
    /// ```typescript
    /// let matrix = ones(3, 5);
    /// assert_eq(size(matrix), [3, 5]);
    /// ```
    /// ```typescript
    /// let matrix = [[[1, 2]]];
    /// assert_eq(size(matrix), [1, 1, 2]);
    /// ```
    #[rhai_fn(name = "size", pure)]
    pub fn matrix_size_by_reference(matrix: &mut Array) -> Array {
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

    /// Return the number of dimensions in matrix, passed by reference.
    /// ```typescript
    /// let matrix = ones(4, 6);
    /// let n = ndims(matrix);
    /// assert_eq(n, 2);
    /// ```
    #[rhai_fn(name = "ndims")]
    pub fn ndims_by_reference(matrix: &mut Array) -> INT {
        matrix_size_by_reference(matrix).len() as INT
    }

    /// Returns the number of elements in a matrix, passed by reference.
    /// ```typescript
    /// let matrix = ones(4, 6);
    /// let n = numel(matrix);
    /// assert_eq(n, 24);
    /// ```
    /// ```typescript
    /// let matrix = [1, [1, 2, 3]];
    /// let n = numel(matrix);
    /// assert_eq(n, 4);
    /// ```
    #[rhai_fn(name = "numel", pure)]
    pub fn numel_by_reference(matrix: &mut Array) -> INT {
        flatten(matrix).len() as INT
    }

    #[cfg(feature = "io")]
    pub mod read_write {
        use nalgebra::DMatrix;
        use polars::prelude::{CsvReader, DataType, SerReader};
        use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Map, Position, FLOAT, INT};
        use std::collections::BTreeMap;

        /// Reads a numeric csv file from a url
        /// ```typescript
        /// let url = "https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv";
        /// let x = read_matrix(url);
        /// assert_eq(size(x), [768, 9]);
        /// ```
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
                        let col: Vec<FLOAT> = series
                            .cast(&DataType::Float64)
                            .expect("Cannot cast to FLOAT")
                            .f64()
                            .unwrap()
                            .into_no_null_iter()
                            .map(|el| el as FLOAT)
                            .collect();
                        final_output.push(col);
                    }

                    final_output = super::super::transpose_internal(final_output);

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

    /// Return a matrix of zeros. Can be called with a single integer argument (indicating the
    /// square matrix of that size) or with an array argument (indicating the size for each dimension).
    /// ```typescript
    /// let matrix = zeros(3);
    /// assert_eq(matrix, [[0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0]]);
    /// ```
    /// ```typescript
    /// let matrix = zeros([3, 3]);
    /// assert_eq(matrix, [[0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0]]);
    /// ```
    /// ```typescript
    /// let matrix = zeros([3, 3, 3]);
    /// assert_eq(matrix, [[[0.0, 0.0, 0.0],
    ///                     [0.0, 0.0, 0.0],
    ///                     [0.0, 0.0, 0.0]],
    ///                    [[0.0, 0.0, 0.0],
    ///                     [0.0, 0.0, 0.0],
    ///                     [0.0, 0.0, 0.0]],
    ///                    [[0.0, 0.0, 0.0],
    ///                     [0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0]]]);
    /// ```
    #[rhai_fn(name = "zeros", return_raw)]
    pub fn zeros_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<INT>() {
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

    /// Return a matrix of zeros. Arguments indicate the number of rows and columns in the matrix.
    /// ```typescript
    /// let matrix = zeros(3, 3);
    /// assert_eq(matrix, [[0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "zeros")]
    pub fn zeros_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for i in 0..nx {
            output.push(Dynamic::from_array(vec![Dynamic::FLOAT_ZERO; ny as usize]))
        }
        output
    }

    /// Return a matrix of ones. Can be called with a single integer argument (indicating the
    /// square matrix of that size) or with an array argument (indicating the size for each dimension).
    /// ```typescript
    /// let matrix = ones(3);
    /// assert_eq(matrix, [[1.0, 1.0, 1.0],
    ///                    [1.0, 1.0, 1.0],
    ///                    [1.0, 1.0, 1.0]]);
    /// ```
    /// ```typescript
    /// let matrix = ones([3, 3]);
    /// assert_eq(matrix, [[1.0, 1.0, 1.0],
    ///                    [1.0, 1.0, 1.0],
    ///                    [1.0, 1.0, 1.0]]);
    /// ```
    /// ```typescript
    /// let matrix = ones([3, 3, 3]);
    /// assert_eq(matrix, [[[1.0, 1.0, 1.0],
    ///                     [1.0, 1.0, 1.0],
    ///                     [1.0, 1.0, 1.0]],
    ///                    [[1.0, 1.0, 1.0],
    ///                     [1.0, 1.0, 1.0],
    ///                     [1.0, 1.0, 1.0]],
    ///                    [[1.0, 1.0, 1.0],
    ///                     [1.0, 1.0, 1.0],
    ///                     [1.0, 1.0, 1.0]]]);
    /// ```
    #[rhai_fn(name = "ones", return_raw)]
    pub fn ones_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<INT>() {
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

    /// Return a matrix of ones. Arguments indicate the number of rows and columns in the matrix.
    /// ```typescript
    /// let matrix = ones(3, 3);
    /// assert_eq(matrix, [[1.0, 1.0, 1.0],
    ///                    [1.0, 1.0, 1.0],
    ///                    [1.0, 1.0, 1.0]]);
    /// ```
    #[rhai_fn(name = "ones")]
    pub fn ones_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for i in 0..nx {
            output.push(Dynamic::from_array(vec![Dynamic::FLOAT_ONE; ny as usize]))
        }
        output
    }

    /// Returns a matrix of random values, each between zero and one. Can be called with a single integer argument (indicating the
    /// square matrix of that size) or with an array argument (indicating the size for each dimension).
    /// ```typescript
    /// let matrix = rand(3);
    /// assert_eq(size(matrix), [3, 3]);
    /// ```
    /// ```typescript
    /// let matrix = rand([3, 3]);
    /// assert_eq(size(matrix), [3, 3]);
    /// ```
    #[rhai_fn(name = "rand", return_raw)]
    pub fn rand_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<INT>() {
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

    /// Return a matrix of random values, each between zero and one. Arguments indicate the number
    /// of rows and columns in the matrix.
    /// ```typescript
    /// let matrix = rand(3, 3);
    /// assert_eq(size(matrix), [3, 3]);
    /// ```
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

    /// Returns an identity matrix. If argument is a single number, then the output is
    /// a square matrix. The argument can also be an array specifying the dimensions separately.
    /// ```typescript
    /// let matrix = eye(3);
    /// assert_eq(matrix, [[1.0, 0.0, 0.0],
    ///                    [0.0, 1.0, 0.0],
    ///                    [0.0, 0.0, 1.0]]);
    /// ```
    /// ```typescript
    /// let matrix = eye([3, 4]);
    /// assert_eq(matrix, [[1.0, 0.0, 0.0, 0.0],
    ///                    [0.0, 1.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 1.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "eye", return_raw)]
    pub fn eye_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        if n.is::<INT>() {
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

    /// Returns the identity matrix, specifying the number of rows and columns separately.
    /// ```typescript
    /// let matrix = eye(3, 4);
    /// assert_eq(matrix, [[1.0, 0.0, 0.0, 0.0],
    ///                    [0.0, 1.0, 0.0, 0.0],
    ///                    [0.0, 0.0, 1.0, 0.0]]);
    /// ```
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
    /// ```typescript
    /// let matrix = rand(3, 5);
    /// let flat = flatten(matrix);
    /// assert_eq(len(flat), 15);
    /// ```
    /// ```typescript
    /// let matrix = [[1.0, 2.0, 3.0], [1.0]];
    /// let flat = flatten(matrix);
    /// assert_eq(len(flat), 4);
    /// ```
    #[rhai_fn(name = "flatten", pure)]
    pub fn flatten(matrix: &mut Array) -> Array {
        let mut flat: Vec<Dynamic> = vec![];
        for el in matrix {
            if el.is::<Array>() {
                flat.extend(flatten(&mut el.clone().into_array().unwrap()))
            } else {
                flat.push(el.clone());
            }
        }
        flat
    }

    /// Flip a matrix left-to-right
    /// ```typescript
    /// let matrix = fliplr([[1.0, 0.0],
    ///                      [0.0, 2.0]]);
    /// assert_eq(matrix, [[0.0, 1.0],
    ///                    [2.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "fliplr", return_raw)]
    pub fn fliplr(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims_by_reference(&mut matrix.clone()) > 2 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be flipped - too many dims"),
                Position::NONE,
            )
            .into())
        } else {
            // Turn into Vec<Vec<FLOAT>>
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
    /// ```typescript
    /// let matrix = flipud([[1.0, 0.0],
    ///                      [0.0, 2.0]]);
    /// assert_eq(matrix, [[0.0, 2.0],
    ///                    [1.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "flipud", return_raw)]
    pub fn flipud(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims_by_reference(&mut matrix.clone()) > 2 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be flipped - too many dims"),
                Position::NONE,
            )
            .into())
        } else {
            // Turn into Vec<Vec<FLOAT>>
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

    /// Rotate a matrix counterclockwise once
    /// ```typescript
    /// let matrix = rot90([[1.0, 0.0],
    ///                    [0.0, 2.0]]);
    /// assert_eq(matrix, [[0.0, 2.0],
    ///                   [1.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "rot90", return_raw)]
    pub fn rot90_once(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims_by_reference(&mut matrix.clone()) == 1 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be rotated - not enough dimensions"),
                Position::NONE,
            )
            .into())
        } else if ndims_by_reference(&mut matrix.clone()) > 2 {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Matrix cannot be rotated - too many dimensions"),
                Position::NONE,
            )
            .into())
        } else {
            // Turn into Vec<Vec<Dynamic>>
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

    /// Rotate a matrix counterclockwise `k` times
    /// ```typescript
    /// let matrix = rot90([[1.0, 0.0],
    ///                     [0.0, 2.0]], 2);
    /// assert_eq(matrix, [[2.0, 0.0],
    ///                    [0.0, 1.0]]);
    /// ```
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

    /// Perform matrix multiplication.
    /// ```typescript
    /// let a = eye(3);
    /// let b = ones(3);
    /// let c = mtimes(a, b);
    /// assert_eq(b, c);
    /// ```
    #[rhai_fn(name = "mtimes", return_raw)]
    pub fn mtimes(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if matrix_size_by_reference(&mut matrix1.clone())[1]
            .as_int()
            .unwrap()
            != matrix_size_by_reference(&mut matrix2.clone())[0]
                .as_int()
                .unwrap()
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

        // Try to multiply
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

    /// Concatenate two arrays horizontally.
    /// ```typescript
    /// let arr1 = rand(3);
    /// let arr2 = rand(3);
    /// let combined = horzcat(arr1, arr2);
    /// assert_eq(size(combined), [3, 6]);
    /// ```
    #[rhai_fn(name = "horzcat", return_raw)]
    pub fn horzcat(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if matrix_size_by_reference(&mut matrix1.clone())[0]
            .as_int()
            .unwrap()
            != matrix_size_by_reference(&mut matrix2.clone())[0]
                .as_int()
                .unwrap()
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

    /// Concatenates two array vertically.
    /// ```typescript
    /// let arr1 = rand(3);
    /// let arr2 = rand(3);
    /// let combined = vertcat(arr1, arr2);
    /// assert_eq(size(combined), [6, 3]);
    /// ```
    #[rhai_fn(name = "vertcat", return_raw)]
    pub fn vertcat(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if matrix_size_by_reference(&mut matrix1.clone())[1]
            .as_int()
            .unwrap()
            != matrix_size_by_reference(&mut matrix2.clone())[1]
                .as_int()
                .unwrap()
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
    /// ```typescript
    ///  let matrix = [[1, 2, 3],
    ///                [4, 5, 6],
    ///                [7, 8, 9]];
    ///  let d = diag(matrix);
    ///  assert_eq(d, [1, 5, 9]);
    /// ```
    /// ```typescript
    ///  let diagonal = [1.0, 2.0, 3.0];
    ///  let matrix = diag(diagonal);
    ///  assert_eq(matrix, [[1.0, 0.0, 0.0],
    ///                     [0.0, 2.0, 0.0],
    ///                     [0.0, 0.0, 3.0]]);
    /// ```
    #[rhai_fn(name = "diag", return_raw)]
    pub fn diag(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        if ndims_by_reference(&mut matrix.clone()) == 2 {
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
        } else if ndims_by_reference(&mut matrix.clone()) == 1 {
            let mut out = vec![];
            for idx in 0..matrix.len() {
                let mut new_row = vec![];
                for jdx in 0..matrix.len() {
                    if idx == jdx {
                        new_row.push(matrix[idx].clone());
                    } else {
                        if matrix[idx].is::<INT>() {
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

    /// Repeats copies of a matrix
    /// ```typescript
    /// let matrix = rand(3);
    /// let combined = repmat(matrix, 2, 2);
    /// assert_eq(size(combined), [6, 6]);
    /// ```
    #[rhai_fn(name = "repmat", return_raw)]
    pub fn repmat(matrix: Array, nx: INT, ny: INT) -> Result<Array, Box<EvalAltResult>> {
        let mut row_matrix = matrix.clone();
        for i in 1..ny {
            match horzcat(row_matrix, matrix.clone()) {
                Ok(mat) => row_matrix = mat,
                Err(e) => return Err(e),
            };
        }
        let mut new_matrix = row_matrix.clone();
        for i in 1..nx {
            match vertcat(new_matrix, row_matrix.clone()) {
                Ok(mat) => new_matrix = mat,
                Err(e) => return Err(e),
            }
        }
        Ok(new_matrix)
    }

    /// Returns an object map containing 2-D grid coordinates based on the uni-axial coordinates
    /// contained in arguments x and y.
    /// ```typescript
    /// let x = [1, 2];
    /// let y = [3, 4];
    /// let g = meshgrid(x, y);
    /// assert_eq(g, #{"x": [[1, 2],
    ///                      [1, 2]],
    ///                "y": [[3, 3],
    ///                      [4, 4]]});
    /// ```
    #[rhai_fn(name = "meshgrid", return_raw)]
    pub fn meshgrid(x: Array, y: Array) -> Result<Map, Box<EvalAltResult>> {
        let nx = x.len();
        let ny = y.len();
        let mut x_dyn: Vec<Dynamic> = vec![Dynamic::from_array(x); nx];
        let mut y_dyn: Vec<Dynamic> = vec![Dynamic::from_array(y); ny];

        let mut result = BTreeMap::new();
        let mut xid = smartstring::SmartString::new();
        xid.push_str("x");
        let mut yid = smartstring::SmartString::new();
        yid.push_str("y");
        result.insert(xid, Dynamic::from_array(x_dyn));
        result.insert(yid, Dynamic::from_array(transpose(y_dyn)));
        Ok(result)
    }

    /// Returns an array containing a number of elements linearly spaced between two bounds.
    /// ```typescript
    /// let x = linspace(1, 2, 5);
    /// assert_eq(x, [1.0, 1.25, 1.5, 1.75, 2.0]);
    /// ```
    #[rhai_fn(name = "linspace", return_raw)]
    pub fn linspace(x1: Dynamic, x2: Dynamic, n: INT) -> Result<Array, Box<EvalAltResult>> {
        let x1_type = x1.type_name();
        let x2_type = x2.type_name();
        if x1_type != x2_type {
            return Err(EvalAltResult::ErrorArithmetic(
                format!(
                    "The left endpoint ({}) and right endpoint ({}) do not have the same type.",
                    x1_type, x2_type
                ),
                Position::NONE,
            )
            .into());
        }

        let new_n = n as FLOAT;
        let mut new_x1 = 0 as FLOAT;
        let mut new_x2 = 0 as FLOAT;

        if x1.is::<FLOAT>() {
            new_x1 = x1.as_float().unwrap();
            new_x2 = x2.as_float().unwrap();
        } else if x1.is::<INT>() {
            new_x1 = x1.as_int().unwrap() as FLOAT;
            new_x2 = x2.as_int().unwrap() as FLOAT;
        } else {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into());
        }

        let mut arr = vec![Dynamic::from_float(new_x1)];
        let mut counter = new_x1;
        let interval = (new_x2 - new_x1) / (new_n - 1.0);
        for i in 0..(n - 2) {
            counter += interval;
            arr.push(Dynamic::from_float(counter));
        }
        arr.push(Dynamic::from_float(new_x2));
        Ok(arr)
    }

    /// Returns an array containing a number of elements logarithmically spaced between two bounds.
    /// ```typescript
    /// let x = logspace(1, 3, 3);
    /// assert_eq(x, [10.0, 100.0, 1000.0]);
    /// ```
    #[rhai_fn(name = "logspace", return_raw)]
    pub fn logspace(a: Dynamic, b: Dynamic, n: INT) -> Result<Array, Box<EvalAltResult>> {
        match linspace(a, b, n) {
            Ok(arr) => Ok(arr
                .iter()
                .map(|e| Dynamic::from_float((10 as FLOAT).powf(e.as_float().unwrap())))
                .collect::<Vec<Dynamic>>()),
            Err(e) => Err(e),
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
