use rhai::plugin::*;

#[export_module]
pub mod matrix_functions {
    #[cfg(feature = "nalgebra")]
    use crate::matrix::RhaiVector;
    use crate::matrix::{numeric_vector_data, RhaiMatrix};
    use crate::validation_functions::{is_column_vector, is_row_vector};
    use crate::{
        array_to_vec_float, if_int_convert_to_float_and_do, if_int_do_else_if_array_do, if_list_do,
        if_matrix_convert_to_vec_array_and_do,
    };
    #[cfg(feature = "nalgebra")]
    use crate::{if_matrices_and_compatible_convert_to_vec_array_and_do, FOIL};
    #[cfg(feature = "nalgebra")]
    use nalgebralib::DMatrix;
    use rhai::{Array, Dynamic, EvalAltResult, Map, Position, FLOAT, INT};
    use std::collections::BTreeMap;

    /// Construct a numeric row vector (1 by N) from a list or vector.
    /// ```typescript
    /// assert_eq(row([1, 2, 3]), [[1, 2, 3]]);
    /// ```
    #[rhai_fn(name = "row", return_raw)]
    pub fn row_from_array(values: Array) -> Result<Array, Box<EvalAltResult>> {
        Ok(RhaiMatrix::row_vector(numeric_vector_data(&values)?).to_array())
    }

    /// Construct a numeric column vector (N by 1) from a list or vector.
    /// ```typescript
    /// assert_eq(col([1, 2, 3]), [[1], [2], [3]]);
    /// ```
    #[rhai_fn(name = "col", return_raw)]
    pub fn col_from_array(values: Array) -> Result<Array, Box<EvalAltResult>> {
        Ok(RhaiMatrix::column_vector(numeric_vector_data(&values)?).to_array())
    }

    /// Validate a nonempty rectangular numeric matrix, preserving INT and FLOAT values.
    /// The result is an ordinary Rhai array; later edits are validated by each operation.
    /// ```typescript
    /// assert_eq(mat([[1, 2.5], [3, 4]]), [[1, 2.5], [3, 4]]);
    /// ```
    #[rhai_fn(name = "mat", return_raw)]
    pub fn mat_from_array(values: Array) -> Result<Array, Box<EvalAltResult>> {
        if crate::matrix::matrix_dimensions(&values).is_none() {
            return Err(EvalAltResult::ErrorArithmetic(
                "mat expects nonempty row arrays of equal length".into(),
                Position::NONE,
            )
            .into());
        }
        for row in &values {
            numeric_vector_data(&row.clone().into_array().unwrap())?;
        }
        Ok(values)
    }

    /// Compute a real scalar inner product of equal-length numeric vectors.
    /// Accepts lists, rows, and columns in any combination and returns FLOAT.
    /// Use `mtimes` for matrix multiplication; matrices with multiple rows and columns
    /// are not accepted by this vector-only function.
    /// ```typescript
    /// assert_eq(dot(row([1, 2]), col([3, 4])), 11.0);
    /// ```
    /// ```typescript
    /// assert_eq(dot(col([1, 2]), col([3, 4])), 11.0);
    /// ```
    #[rhai_fn(name = "dot", return_raw)]
    pub fn dot(left: Array, right: Array) -> Result<FLOAT, Box<EvalAltResult>> {
        let mut left = numeric_vector_data(&left)?;
        let mut right = numeric_vector_data(&right)?;
        if left.len() != right.len() {
            return Err(EvalAltResult::ErrorArithmetic(
                "dot expects vectors of the same length".into(),
                Position::NONE,
            )
            .into());
        }
        Ok(array_to_vec_float(&mut left)
            .iter()
            .zip(array_to_vec_float(&mut right))
            .map(|(a, b)| a * b)
            .sum())
    }

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
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "inv", return_raw, pure)]
    pub fn invert_matrix(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        let dm = RhaiMatrix::from_array(matrix.clone()).to_dmatrix()?;
        dm.try_inverse()
            .map(|m| RhaiMatrix::from_dmatrix(&m).to_array())
            .ok_or_else(|| {
                EvalAltResult::ErrorArithmetic(
                    "Matrix cannot be inverted".to_string(),
                    Position::NONE,
                )
                .into()
            })
    }

    /// Calculate the eigenvalues and eigenvectors for a matrix. Specifically, the output is an
    /// object map with entries for real_eigenvalues, imaginary_eigenvalues, eigenvectors, and
    /// residuals.
    /// ```typescript
    /// let matrix = eye(5);
    /// let eig = eigs(matrix);
    /// assert(sum(eig.residuals) < 0.000001);
    /// ```
    /// ```typescript
    /// let matrix = [[ 0.0,  1.0],
    ///               [-2.0, -3.0]];
    /// let eig = eigs(matrix);
    /// assert(sum(eig.residuals) < 0.000001);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "eigs", return_raw, pure)]
    pub fn matrix_eigs_alt(matrix: &mut Array) -> Result<Map, Box<EvalAltResult>> {
        let dm = RhaiMatrix::from_array(matrix.clone()).to_dmatrix()?;

        let dms = dm.shape().1;

        let eigenvalues = dm.complex_eigenvalues();

        let mut imaginary_values = vec![Dynamic::from_float(1.0); 0];
        let mut real_values = vec![Dynamic::from_float(1.0); 0];
        let mut residuals = vec![Dynamic::from_float(1.0); 0];
        let mut eigenvectors = DMatrix::from_element(dms, 0, 0.0);
        for (idx, ev) in eigenvalues.iter().enumerate() {
            imaginary_values.push(Dynamic::from_float(ev.im));
            real_values.push(Dynamic::from_float(ev.re));

            let mut a = dm.clone() - DMatrix::from_diagonal_element(dms, dms, ev.re);
            a = a.insert_column(0, 0.0);
            a = a.insert_row(0, 0.0);
            a[(0, idx + 1)] = 1.0;
            let mut b = DMatrix::from_element(dms + 1, 1, 0.0);
            b[(0, 0)] = 1.0;
            let eigenvector = a
                .svd(true, true)
                .solve(&b, 1e-10)
                .unwrap()
                .remove_rows(0, 1)
                .normalize();

            residuals.push(Dynamic::from_float(
                (dm.clone() * eigenvector.clone() - ev.re * eigenvector.clone()).amax(),
            ));

            eigenvectors.extend(eigenvector.column_iter());
        }

        let mut result = BTreeMap::new();
        let mut vid = smartstring::SmartString::new();
        vid.push_str("eigenvectors");
        result.insert(
            vid,
            Dynamic::from_array(RhaiMatrix::from_dmatrix(&eigenvectors).to_array()),
        );
        let mut did = smartstring::SmartString::new();
        did.push_str("real_eigenvalues");
        result.insert(did, Dynamic::from_array(real_values));
        let mut eid = smartstring::SmartString::new();
        eid.push_str("imaginary_eigenvalues");
        result.insert(eid, Dynamic::from_array(imaginary_values));
        let mut rid = smartstring::SmartString::new();
        rid.push_str("residuals");
        result.insert(rid, Dynamic::from_array(residuals));

        Ok(result)
    }

    /// Calculates the singular value decomposition of a matrix
    /// ```typescript
    /// let matrix = eye(5);
    /// let svd_results = svd(matrix);
    /// assert_eq(svd_results, #{"s": ones([5]), "u": eye(5), "v": eye(5)});
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "svd", return_raw, pure)]
    pub fn svd_decomp(matrix: &mut Array) -> Result<Map, Box<EvalAltResult>> {
        let dm = RhaiMatrix::from_array(matrix.clone()).to_dmatrix()?;
        let svd = nalgebralib::linalg::SVD::new(dm, true, true);

        let mut result = BTreeMap::new();
        let mut u_key = smartstring::SmartString::new();
        u_key.push_str("u");
        match svd.u {
            Some(u) => result.insert(
                u_key,
                Dynamic::from_array(RhaiMatrix::from_dmatrix(&u).to_array()),
            ),
            None => {
                return Err(EvalAltResult::ErrorArithmetic(
                    "SVD decomposition cannot be computed for this matrix.".to_string(),
                    Position::NONE,
                )
                .into());
            }
        };

        let mut v_key = smartstring::SmartString::new();
        v_key.push_str("v");
        match svd.v_t {
            Some(v) => result.insert(
                v_key,
                Dynamic::from_array(RhaiMatrix::from_dmatrix(&v).to_array()),
            ),
            None => {
                return Err(EvalAltResult::ErrorArithmetic(
                    "SVD decomposition cannot be computed for this matrix.".to_string(),
                    Position::NONE,
                )
                .into());
            }
        };

        let mut s_key = smartstring::SmartString::new();
        s_key.push_str("s");
        result.insert(
            s_key,
            Dynamic::from_array(RhaiVector::from_dvector(&svd.singular_values).to_array()),
        );

        Ok(result)
    }

    /// Calculates the QR decomposition of a matrix
    /// ```typescript
    /// let matrix = eye(5);
    /// let qr_results = qr(matrix);
    /// assert_eq(qr_results, #{"q": eye(5), "r": eye(5)});
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "qr", return_raw, pure)]
    pub fn qr_decomp(matrix: &mut Array) -> Result<Map, Box<EvalAltResult>> {
        let dm = RhaiMatrix::from_array(matrix.clone()).to_dmatrix()?;
        let qr = nalgebralib::linalg::QR::new(dm);

        let mut result = BTreeMap::new();
        let mut qid = smartstring::SmartString::new();
        qid.push_str("q");
        result.insert(
            qid,
            Dynamic::from_array(RhaiMatrix::from_dmatrix(&qr.q()).to_array()),
        );

        let mut rid = smartstring::SmartString::new();
        rid.push_str("r");
        result.insert(
            rid,
            Dynamic::from_array(RhaiMatrix::from_dmatrix(&qr.r()).to_array()),
        );

        Ok(result)
    }

    /// Calculates the QR decomposition of a matrix
    /// ```typescript
    /// let matrix = eye(5);
    /// let h_results = hessenberg(matrix);
    /// assert_eq(h_results, #{"h": eye(5), "q": eye(5)});
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "hessenberg", return_raw, pure)]
    pub fn hessenberg(matrix: &mut Array) -> Result<Map, Box<EvalAltResult>> {
        let dm = RhaiMatrix::from_array(matrix.clone()).to_dmatrix()?;
        let h = nalgebralib::linalg::Hessenberg::new(dm);

        let mut result = BTreeMap::new();
        let mut hid = smartstring::SmartString::new();
        hid.push_str("h");
        result.insert(
            hid,
            Dynamic::from_array(RhaiMatrix::from_dmatrix(&h.h()).to_array()),
        );

        let mut qid = smartstring::SmartString::new();
        qid.push_str("q");
        result.insert(
            qid,
            Dynamic::from_array(RhaiMatrix::from_dmatrix(&h.q()).to_array()),
        );

        Ok(result)
    }

    /// Transposes a matrix.
    /// ```typescript
    /// let row = [[1, 2, 3, 4]];
    /// let column = transpose(row);
    /// assert_eq(column, [[1.0],
    ///                    [2.0],
    ///                    [3.0],
    ///                    [4.0]]);
    /// ```
    /// ```typescript
    /// let matrix = transpose(eye(3));
    /// assert_eq(matrix, eye(3));
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "transpose", return_raw)]
    pub fn transpose(matrix: RhaiMatrix) -> Result<RhaiMatrix, Box<EvalAltResult>> {
        let mut raw = matrix.clone().to_array();
        if !raw.is_empty() && matrix_size_by_reference(&mut raw).len() == 1 {
            return RhaiMatrix::row_vector(raw).transpose();
        }

        matrix.transpose()
    }

    /// Transpose an array by first converting it to a [`RhaiMatrix`].
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "transpose", return_raw)]
    pub fn transpose_from_array(matrix: Array) -> Result<Array, Box<EvalAltResult>> {
        transpose(RhaiMatrix::from_array(matrix)).map(RhaiMatrix::to_array)
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
        while let Some(first) = new_matrix.first() {
            if first.is_array() {
                new_matrix = first.clone().into_array().unwrap();
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

    /// Returns the number of non-zero elements in a matrix, passed by reference.
    /// ```typescript
    /// let matrix = ones(4, 6);
    /// let n = nnz(matrix);
    /// assert_eq(n, 24);
    /// ```
    /// ```typescript
    /// let matrix = eye(4);
    /// let n = nnz(matrix);
    /// assert_eq(n, 4);
    /// ```
    #[rhai_fn(name = "nnz", pure)]
    pub fn nnz_by_reference(matrix: &mut Array) -> INT {
        array_to_vec_float(&mut flatten(matrix))
            .iter()
            .filter(|&n| *n > 0.0)
            .count() as INT
    }

    #[cfg(feature = "io")]
    pub mod read_write {
        use polars::prelude::{CsvReadOptions, DataType, SerReader};
        use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, FLOAT};

        /// Reads a numeric CSV file from the filesystem
        /// ```typescript
        /// let x = read_matrix("tests/fixtures/sample_matrix.csv");
        /// assert_eq(x, [[1.0, 2.0], [3.0, 4.0]]);
        /// ```
        #[rhai_fn(name = "read_matrix", return_raw)]
        pub fn read_matrix(file_path: ImmutableString) -> Result<Array, Box<EvalAltResult>> {
            // We will use this function later
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

            let file_path_as_str = file_path.as_str();

            // Determine path is url
            let path_is_url = url::Url::parse(file_path_as_str);

            match path_is_url {
                Err(_) => {
                    let x = CsvReadOptions::default()
                        .with_has_header(
                            csv_sniffer::Sniffer::new()
                                .sniff_path(file_path_as_str)
                                .map_err(|err| {
                                    EvalAltResult::ErrorSystem(
                                        format!("Cannot sniff file: {file_path_as_str}"),
                                        err.into(),
                                    )
                                })?
                                .dialect
                                .header
                                .has_header_row,
                        )
                        .try_into_reader_with_file_path(Some(file_path_as_str.into()))
                        .map_err(|err| {
                            EvalAltResult::ErrorSystem(
                                format!("Cannot read file as CSV: {file_path_as_str}"),
                                err.into(),
                            )
                        })?
                        .finish()
                        .map_err(|err| {
                            EvalAltResult::ErrorSystem(
                                format!("Cannot read file: {file_path_as_str}"),
                                err.into(),
                            )
                        })?
                        .drop_nulls::<String>(None)
                        .map_err(|err| {
                            EvalAltResult::ErrorSystem(
                                format!("Cannot remove null values from file: {file_path_as_str}"),
                                err.into(),
                            )
                        })?;

                    // Convert into vec of vec
                    let mut final_output = vec![];
                    for series in x.get_columns() {
                        let col: Vec<FLOAT> = series
                            .cast(&DataType::Float64)
                            .map_err(|err| {
                                EvalAltResult::ErrorArithmetic(
                                    format!("Data cannot be cast to FLOAT: {err}"),
                                    rhai::Position::NONE,
                                )
                            })?
                            .f64()
                            .unwrap()
                            .into_no_null_iter()
                            .map(|el| el as FLOAT)
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
                        .collect::<Array>();

                    Ok(matrix_as_array)
                }
                Ok(_) => {
                    if let Ok(_) = url::Url::parse(file_path_as_str) {
                        let file_contents =
                            minreq::get(file_path_as_str).send().map_err(|err| {
                                EvalAltResult::ErrorSystem(
                                    format!("Error getting url: {file_path_as_str}"),
                                    err.into(),
                                )
                            })?;
                        let temp = temp_file::with_contents(file_contents.as_bytes());

                        let temp_file_name: ImmutableString = temp.path().to_str().unwrap().into();

                        read_matrix(temp_file_name)
                    } else {
                        EvalAltResult::ErrorRuntime(
                            format!(
                                "The string {file_path_as_str} is not a valid URL or file path",
                            )
                            .into(),
                            rhai::Position::NONE,
                        )
                        .into()
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
    /// let matrix = zeros([3]);
    /// assert_eq(matrix, [0.0, 0.0, 0.0]);
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
        if_int_do_else_if_array_do(
            n,
            |n| Ok(zeros_double_input(n, n)),
            |m| {
                if m.len() == 2 {
                    Ok(zeros_double_input(
                        m[0].as_int().unwrap(),
                        m[1].as_int().unwrap(),
                    ))
                } else if m.len() > 2 {
                    let l = m.remove(0);
                    Ok(vec![
                        Dynamic::from_array(
                            zeros_single_input(Dynamic::from_array(m.to_vec())).unwrap()
                        );
                        l.as_int().unwrap() as usize
                    ])
                } else {
                    Ok(vec![Dynamic::FLOAT_ZERO; m[0].as_int().unwrap() as usize])
                }
            },
        )
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
        for _ in 0..nx {
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
    /// let matrix = ones([3]);
    /// assert_eq(matrix, [1.0, 1.0, 1.0]);
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
        crate::if_int_do_else_if_array_do(
            n,
            |n| Ok(ones_double_input(n, n)),
            |m| {
                if m.len() == 2 {
                    Ok(ones_double_input(
                        m[0].as_int().unwrap(),
                        m[1].as_int().unwrap(),
                    ))
                } else if m.len() > 2 {
                    let l = m.remove(0);
                    Ok(vec![
                        Dynamic::from_array(
                            ones_single_input(Dynamic::from_array(m.to_vec())).unwrap()
                        );
                        l.as_int().unwrap() as usize
                    ])
                } else {
                    Ok(vec![Dynamic::FLOAT_ONE; m[0].as_int().unwrap() as usize])
                }
            },
        )
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
        for _ in 0..nx {
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
    #[cfg(feature = "rand")]
    #[rhai_fn(name = "rand", return_raw)]
    pub fn rand_single_input(n: Dynamic) -> Result<Array, Box<EvalAltResult>> {
        crate::if_int_do_else_if_array_do(
            n,
            |n| Ok(rand_double_input(n, n)),
            |m| {
                if m.len() == 2 {
                    Ok(rand_double_input(
                        m[0].as_int().unwrap(),
                        m[1].as_int().unwrap(),
                    ))
                } else if m.len() > 2 {
                    let l = m.remove(0);
                    Ok(vec![
                        Dynamic::from_array(
                            rand_single_input(Dynamic::from_array(m.to_vec())).unwrap()
                        );
                        l.as_int().unwrap() as usize
                    ])
                } else {
                    Ok(rand_double_input(1, m[0].as_int().unwrap())[0]
                        .clone()
                        .into_array()
                        .unwrap())
                }
            },
        )
    }

    /// Return a matrix of random values, each between zero and one. Arguments indicate the number
    /// of rows and columns in the matrix.
    /// ```typescript
    /// let matrix = rand(3, 3);
    /// assert_eq(size(matrix), [3, 3]);
    /// ```
    #[cfg(feature = "rand")]
    #[rhai_fn(name = "rand")]
    pub fn rand_double_input(nx: INT, ny: INT) -> Array {
        let mut output = vec![];
        for _ in 0..nx {
            let mut row = vec![];
            for _ in 0..ny {
                row.push(Dynamic::from_float(crate::misc_functions::rand_float()));
            }
            output.push(Dynamic::from_array(row))
        }
        output
    }

    /// Returns an identity matrix. If argument is a single number, then the output is
    /// a square matrix. The argument can also be an array specifying the dimensions separately.
    /// Passing `[n]` is equivalent to `eye(n)` (a square `n x n` matrix) while `[rows, cols]`
    /// creates a rectangular matrix with the provided row and column counts. Any other shape is
    /// rejected.
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
        fn parse_eye_dimension(value: &Dynamic) -> Result<INT, Box<EvalAltResult>> {
            value.as_int().map_err(|_| {
                EvalAltResult::ErrorMismatchDataType(
                    "Size vector for eye must contain integers".to_string(),
                    String::new(),
                    Position::NONE,
                )
                .into()
            })
        }

        if_int_do_else_if_array_do(
            n,
            |n| Ok(eye_double_input(n, n)),
            |m| match m.len() {
                1 => {
                    let size = parse_eye_dimension(&m[0])?;
                    Ok(eye_double_input(size, size))
                }
                2 => {
                    let rows = parse_eye_dimension(&m[0])?;
                    let cols = parse_eye_dimension(&m[1])?;
                    Ok(eye_double_input(rows, cols))
                }
                _ => Err(EvalAltResult::ErrorMismatchDataType(
                    "Cannot create an identity matrix with more than 2 dimensions".to_string(),
                    String::new(),
                    Position::NONE,
                )
                .into()),
            },
        )
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

    /// Returns the contents of a multidimensional array as a 1-D array.
    /// ```typescript
    /// let matrix = ones(3, 5);
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
        let mut flat: Array = vec![];
        for el in matrix {
            if el.is_array() {
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
    pub fn fliplr(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            let w = matrix_as_vec[0].len();
            let h = matrix_as_vec.len();

            // Turn into Array
            let mut out = vec![];
            for idx in 0..h {
                let mut new_row = vec![];
                for jdx in 0..w {
                    new_row.push(matrix_as_vec[idx][w - jdx - 1].clone());
                }
                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        })
    }

    /// Flip a matrix up-down
    /// ```typescript
    /// let matrix = flipud([[1.0, 0.0],
    ///                      [0.0, 2.0]]);
    /// assert_eq(matrix, [[0.0, 2.0],
    ///                    [1.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "flipud", return_raw)]
    pub fn flipud(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            let w = matrix_as_vec[0].len();
            let h = matrix_as_vec.len();

            // Turn into Array
            let mut out = vec![];
            for idx in 0..h {
                let mut new_row = vec![];
                for jdx in 0..w {
                    new_row.push(matrix_as_vec[h - idx - 1][jdx].clone());
                }
                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        })
    }

    /// Rotate a matrix counterclockwise once
    /// ```typescript
    /// let matrix = rot90([[1.0, 0.0],
    ///                    [0.0, 2.0]]);
    /// assert_eq(matrix, [[0.0, 2.0],
    ///                   [1.0, 0.0]]);
    /// ```
    #[rhai_fn(name = "rot90", return_raw)]
    pub fn rot90_once(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            let w = matrix_as_vec[0].len();
            let h = matrix_as_vec.len();

            // Turn into Array
            let mut out = vec![];
            for idx in 0..w {
                let mut new_row = vec![];
                for jdx in 0..h {
                    new_row.push(matrix_as_vec[jdx][w - idx - 1].clone());
                }

                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        })
    }

    /// Rotate a matrix counterclockwise `k` times
    /// ```typescript
    /// let matrix = rot90([[1.0, 0.0],
    ///                     [0.0, 2.0]], 2);
    /// assert_eq(matrix, [[2.0, 0.0],
    ///                    [0.0, 1.0]]);
    /// ```
    #[rhai_fn(name = "rot90", return_raw)]
    pub fn rot90_ktimes(matrix: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if k <= 0 {
            return Ok(matrix.clone());
        }

        let mut result = matrix;
        let mut result_base = Array::new();

        for _ in 0..k {
            result_base = rot90_once(result)?;
            result = &mut result_base;
        }

        Ok(result_base)
    }

    /// Perform matrix multiplication.
    /// ```typescript
    /// let a = eye(3);
    /// let b = ones(3);
    /// let c = mtimes(a, b);
    /// assert_eq(b, c);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "mtimes", return_raw)]
    pub fn mtimes(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrices_and_compatible_convert_to_vec_array_and_do(
            FOIL::Inside,
            &mut matrix1.clone(),
            &mut matrix2.clone(),
            |matrix_as_vec1, matrix_as_vec2| {
                let arr1: Array = matrix_as_vec1
                    .into_iter()
                    .map(Dynamic::from_array)
                    .collect();
                let arr2: Array = matrix_as_vec2
                    .into_iter()
                    .map(Dynamic::from_array)
                    .collect();
                let dm1 = RhaiMatrix::from_array(arr1).to_dmatrix()?;
                let dm2 = RhaiMatrix::from_array(arr2).to_dmatrix()?;
                let mat = dm1 * dm2;
                Ok(RhaiMatrix::from_dmatrix(&mat).to_array())
            },
        )
    }

    /// Concatenate two arrays horizontally.
    /// ```typescript
    /// let left = [[1, 2]];
    /// let right = [[3, 4]];
    /// let row = horzcat(left, right);
    /// assert_eq(row, [[1.0, 2.0, 3.0, 4.0]]);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "horzcat", return_raw)]
    pub fn horzcat(
        matrix1: RhaiMatrix,
        matrix2: RhaiMatrix,
    ) -> Result<RhaiMatrix, Box<EvalAltResult>> {
        matrix1.concat_h(&matrix2)
    }

    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "horzcat", return_raw)]
    pub fn horzcat_from_array(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        horzcat(
            RhaiMatrix::from_array(matrix1),
            RhaiMatrix::from_array(matrix2),
        )
        .map(RhaiMatrix::to_array)
    }

    /// Concatenates two array vertically.
    /// ```typescript
    /// let top = [[1], [2]];
    /// let bottom = [[3], [4]];
    /// let column = vertcat(top, bottom);
    /// assert_eq(column, [[1.0], [2.0], [3.0], [4.0]]);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "vertcat", return_raw)]
    pub fn vertcat(
        matrix1: RhaiMatrix,
        matrix2: RhaiMatrix,
    ) -> Result<RhaiMatrix, Box<EvalAltResult>> {
        matrix1.concat_v(&matrix2)
    }

    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "vertcat", return_raw)]
    pub fn vertcat_from_array(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        vertcat(
            RhaiMatrix::from_array(matrix1),
            RhaiMatrix::from_array(matrix2),
        )
        .map(RhaiMatrix::to_array)
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
        let dims = ndims_by_reference(&mut matrix.clone());
        if dims == 2 {
            let mut candidate_for_row = matrix.clone();
            let mut candidate_for_col = matrix.clone();
            if is_row_vector(&mut candidate_for_row) || is_column_vector(&mut candidate_for_col) {
                let mut flattened_vector = matrix.clone();
                let vector = flatten(&mut flattened_vector);
                return Ok(diagonal_matrix_from_vector(vector));
            }

            let matrix_as_vec = matrix
                .into_iter()
                .map(|x| x.into_array().unwrap())
                .collect::<Vec<Array>>();

            if matrix_as_vec.is_empty() {
                return Ok(vec![]);
            }

            let cols = matrix_as_vec[0].len();
            let diag_len = matrix_as_vec.len().min(cols);
            let mut out = vec![];
            for i in 0..diag_len {
                out.push(matrix_as_vec[i][i].clone());
            }

            Ok(out)
        } else if dims == 1 {
            Ok(diagonal_matrix_from_vector(matrix))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                "Argument must be a 2-D matrix (to extract the diagonal) or a 1-D array (to create a matrix with that diagonal".to_string(),
                Position::NONE,
            )
            .into())
        }
    }

    fn diagonal_matrix_from_vector(vector: Array) -> Array {
        let mut out = vec![];
        for idx in 0..vector.len() {
            let mut new_row = vec![];
            for jdx in 0..vector.len() {
                if idx == jdx {
                    new_row.push(vector[idx].clone());
                } else if vector[idx].is_int() {
                    new_row.push(Dynamic::ZERO);
                } else {
                    new_row.push(Dynamic::FLOAT_ZERO);
                }
            }
            out.push(Dynamic::from_array(new_row));
        }
        out
    }

    /// Repeats copies of a matrix
    /// ```typescript
    /// let matrix = eye(3);
    /// let combined = repmat(matrix, 2, 2);
    /// assert_eq(size(combined), [6, 6]);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "repmat", return_raw)]
    pub fn repmat(matrix: RhaiMatrix, nx: INT, ny: INT) -> Result<RhaiMatrix, Box<EvalAltResult>> {
        let oriented = matrix
            .as_column()
            .or_else(|| matrix.as_row())
            .unwrap_or(matrix);
        let dm = oriented.to_dmatrix()?;
        let nx = if nx < 1 { 1 } else { nx as usize };
        let ny = if ny < 1 { 1 } else { ny as usize };
        let mat = DMatrix::from_fn(dm.nrows() * nx, dm.ncols() * ny, |i, j| {
            dm[(i % dm.nrows(), j % dm.ncols())]
        });
        Ok(RhaiMatrix::from_dmatrix(&mat))
    }

    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "repmat", return_raw)]
    pub fn repmat_from_array(matrix: Array, nx: INT, ny: INT) -> Result<Array, Box<EvalAltResult>> {
        repmat(RhaiMatrix::from_array(matrix), nx, ny).map(RhaiMatrix::to_array)
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
    ///
    /// let x = [0, 1, 2];
    /// let y = [10];
    /// let g = meshgrid(x, y);
    /// assert_eq(g, #{"x": [[0, 1, 2]],
    ///                "y": [[10, 10, 10]]});
    /// ```
    #[rhai_fn(name = "meshgrid", return_raw)]
    pub fn meshgrid(x: Array, y: Array) -> Result<Map, Box<EvalAltResult>> {
        if_list_do(&mut x.clone(), |x| {
            if_list_do(&mut y.clone(), |y| {
                let nx = x.len();
                let ny = y.len();
                let x_dyn: Array = (0..ny).map(|_| Dynamic::from_array(x.to_vec())).collect();
                let y_dyn: Array = y
                    .iter()
                    .map(|value| {
                        let y_row: Array = (0..nx).map(|_| value.clone()).collect();
                        Dynamic::from_array(y_row)
                    })
                    .collect();

                let mut result = BTreeMap::new();
                let mut xid = smartstring::SmartString::new();
                xid.push_str("x");
                let mut yid = smartstring::SmartString::new();
                yid.push_str("y");
                result.insert(xid, Dynamic::from_array(x_dyn));
                result.insert(yid, Dynamic::from_array(y_dyn));
                Ok(result)
            })
        })
    }

    /// Returns an array containing a number of elements linearly spaced between two bounds.
    /// ```typescript
    /// let x = linspace(1, 2, 5);
    /// assert_eq(x, [1.0, 1.25, 1.5, 1.75, 2.0]);
    /// ```
    #[rhai_fn(name = "linspace", return_raw)]
    pub fn linspace(x1: Dynamic, x2: Dynamic, n: INT) -> Result<Array, Box<EvalAltResult>> {
        if_int_convert_to_float_and_do(x1, |new_x1| {
            if_int_convert_to_float_and_do(x2.clone(), |new_x2| {
                let new_n = n as FLOAT;

                let mut arr = vec![Dynamic::from_float(new_x1)];
                let mut counter = new_x1;
                let interval = (new_x2 - new_x1) / (new_n - 1.0);
                for _ in 0..(n - 2) {
                    counter += interval;
                    arr.push(Dynamic::from_float(counter));
                }
                arr.push(Dynamic::from_float(new_x2));
                Ok(arr)
            })
        })
    }

    /// Returns an array containing a number of elements logarithmically spaced between two bounds.
    /// ```typescript
    /// let x = logspace(1, 3, 3);
    /// assert_eq(x, [10.0, 100.0, 1000.0]);
    /// ```
    #[rhai_fn(name = "logspace", return_raw)]
    pub fn logspace(a: Dynamic, b: Dynamic, n: INT) -> Result<Array, Box<EvalAltResult>> {
        linspace(a, b, n).map(|arr| {
            arr.iter()
                .map(|e| Dynamic::from_float((10 as FLOAT).powf(e.as_float().unwrap())))
                .collect::<Array>()
        })
    }
}
