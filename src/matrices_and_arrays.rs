use rhai::plugin::*;
#[cfg(feature = "smartcore")]
use smartcorelib::linalg::evd::*;

#[export_module]
pub mod matrix_functions {
    #[cfg(feature = "smartcore")]
    use crate::{dense_matrix_to_vec_dynamic, if_matrix_convert_to_dense_matrix_and_do};
    use crate::{
        if_int_convert_to_float_and_do, if_int_do_else_if_array_do, if_list_do,
        if_matrices_and_compatible_convert_to_vec_array_and_do,
        if_matrix_convert_to_vec_array_and_do, if_matrix_do, FOIL,
    };
    #[cfg(feature = "nalgebra")]
    use crate::{omatrix_to_vec_dynamic, ovector_to_vec_dynamic};
    #[cfg(feature = "nalgebra")]
    use nalgebralib::DMatrix;
    use rhai::{Array, Dynamic, EvalAltResult, Map, Position, FLOAT, INT};
    #[cfg(feature = "smartcore")]
    use smartcorelib::linalg::BaseMatrix;
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
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "inv", return_raw, pure)]
    pub fn invert_matrix(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
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

                Some(mat) => Ok(omatrix_to_vec_dynamic(mat)),
            }
        })
    }
    /// Calculate the eigenvalues for a matrix.
    /// ```typescript
    /// let matrix = eye(5);
    /// let eig = eigs(matrix);
    /// assert_eq(eig, #{ "eigenvectors": [[1.0, 0.0, 0.0, 0.0, 0.0],
    ///                                    [0.0, 1.0, 0.0, 0.0, 0.0],
    ///                                    [0.0, 0.0, 1.0, 0.0, 0.0],
    ///                                    [0.0, 0.0, 0.0, 1.0, 0.0],
    ///                                    [0.0, 0.0, 0.0, 0.0, 1.0]],
    ///                   "imaginary_eigenvalues": [0.0, 0.0, 0.0, 0.0, 0.0],
    ///                   "real_eigenvalues": [1.0, 1.0, 1.0, 1.0, 1.0]});
    /// ```
    #[cfg(feature = "smartcore")]
    #[rhai_fn(name = "eigs", return_raw, pure)]
    pub fn matrix_eigs(matrix: &mut Array) -> Result<Map, Box<EvalAltResult>> {
        if_matrix_convert_to_dense_matrix_and_do(matrix, |matrix_as_dm| {
            // Try to invert
            let dm =
                matrix_as_dm.evd(matrix_as_dm.approximate_eq(&matrix_as_dm.transpose(), 0.0000001));

            match dm {
                Err(e) => {
                    Err(EvalAltResult::ErrorArithmetic(format!("{:?}", e), Position::NONE).into())
                }

                Ok(evd) => {
                    let vecs: Array = dense_matrix_to_vec_dynamic(evd.V);
                    let real_values: Array = evd
                        .d
                        .into_iter()
                        .map(|x| Dynamic::from_float(x))
                        .collect::<Vec<Dynamic>>();
                    let imaginary_values: Array = evd
                        .e
                        .into_iter()
                        .map(|x| Dynamic::from_float(x))
                        .collect::<Vec<Dynamic>>();

                    let mut result = BTreeMap::new();
                    let mut vid = smartstring::SmartString::new();
                    vid.push_str("eigenvectors");
                    result.insert(vid, Dynamic::from_array(vecs));
                    let mut did = smartstring::SmartString::new();
                    did.push_str("real_eigenvalues");
                    result.insert(did, Dynamic::from_array(real_values));
                    let mut eid = smartstring::SmartString::new();
                    eid.push_str("imaginary_eigenvalues");
                    result.insert(eid, Dynamic::from_array(imaginary_values));

                    Ok(result)
                }
            }
        })
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
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
                if matrix_as_vec[0][0].is::<FLOAT>() {
                    matrix_as_vec[i][j].as_float().unwrap()
                } else {
                    matrix_as_vec[i][j].as_int().unwrap() as FLOAT
                }
            });

            // Try ot invert
            let svd = nalgebralib::linalg::SVD::new(dm, true, true);

            let mut result = BTreeMap::new();
            let mut uid = smartstring::SmartString::new();
            uid.push_str("u");
            match svd.u {
                Some(u) => result.insert(uid, Dynamic::from_array(omatrix_to_vec_dynamic(u))),
                None => {
                    return Err(EvalAltResult::ErrorArithmetic(
                        format!("SVD decomposition cannot be computed for this matrix."),
                        Position::NONE,
                    )
                    .into());
                }
            };

            let mut vid = smartstring::SmartString::new();
            vid.push_str("v");
            match svd.v_t {
                Some(v) => result.insert(vid, Dynamic::from_array(omatrix_to_vec_dynamic(v))),
                None => {
                    return Err(EvalAltResult::ErrorArithmetic(
                        format!("SVD decomposition cannot be computed for this matrix."),
                        Position::NONE,
                    )
                    .into());
                }
            };

            let mut sid = smartstring::SmartString::new();
            sid.push_str("s");
            result.insert(
                sid,
                Dynamic::from_array(ovector_to_vec_dynamic(svd.singular_values)),
            );

            Ok(result)
        })
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
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
                if matrix_as_vec[0][0].is::<FLOAT>() {
                    matrix_as_vec[i][j].as_float().unwrap()
                } else {
                    matrix_as_vec[i][j].as_int().unwrap() as FLOAT
                }
            });

            // Try ot invert
            let qr = nalgebralib::linalg::QR::new(dm);

            let mut result = BTreeMap::new();
            let mut qid = smartstring::SmartString::new();
            qid.push_str("q");
            result.insert(qid, Dynamic::from_array(omatrix_to_vec_dynamic(qr.q())));

            let mut rid = smartstring::SmartString::new();
            rid.push_str("r");
            result.insert(rid, Dynamic::from_array(omatrix_to_vec_dynamic(qr.r())));

            Ok(result)
        })
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
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            let dm = DMatrix::from_fn(matrix_as_vec.len(), matrix_as_vec[0].len(), |i, j| {
                if matrix_as_vec[0][0].is::<FLOAT>() {
                    matrix_as_vec[i][j].as_float().unwrap()
                } else {
                    matrix_as_vec[i][j].as_int().unwrap() as FLOAT
                }
            });

            // Try ot invert
            let h = nalgebralib::linalg::Hessenberg::new(dm);

            let mut result = BTreeMap::new();
            let mut hid = smartstring::SmartString::new();
            hid.push_str("h");
            result.insert(hid, Dynamic::from_array(omatrix_to_vec_dynamic(h.h())));

            let mut qid = smartstring::SmartString::new();
            qid.push_str("q");
            result.insert(qid, Dynamic::from_array(omatrix_to_vec_dynamic(h.q())));

            Ok(result)
        })
    }

    /// Transposes a matrix.
    /// ```typescript
    /// let row = [[1, 2, 3, 4]];
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
    #[rhai_fn(name = "transpose", pure, return_raw)]
    pub fn transpose(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
            // Turn into Vec<Dynamic>
            let mut out = vec![];
            for idx in 0..matrix_as_vec[0].len() {
                let mut new_row = vec![];
                for jdx in 0..matrix_as_vec.len() {
                    new_row.push(matrix_as_vec[jdx][idx].clone());
                }
                out.push(Dynamic::from_array(new_row));
            }
            Ok(out)
        })
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

    #[cfg(all(feature = "io"))]
    pub mod read_write {
        use polars::prelude::{CsvReader, DataType, SerReader};
        use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, FLOAT};

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
                    let l = m[0].clone();
                    m.remove(0);
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
                    let l = m[0].clone();
                    m.remove(0);
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
                    let l = m[0].clone();
                    m.remove(0);
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
        if_int_do_else_if_array_do(
            n,
            |n| Ok(eye_double_input(n, n)),
            |m| {
                if m.len() == 1 {
                    Ok(eye_double_input(1, m[0].as_int().unwrap())[0]
                        .clone()
                        .into_array()
                        .unwrap())
                } else if m.len() == 2 {
                    Ok(eye_double_input(
                        m[0].as_int().unwrap(),
                        m[1].as_int().unwrap(),
                    ))
                } else {
                    Err(EvalAltResult::ErrorMismatchDataType(
                        format!("Cannot create an identity matrix with more than 2 dimensions."),
                        format!(""),
                        Position::NONE,
                    )
                    .into())
                }
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

    /// Returns the contents of an multidimensional array as a 1-D array.
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
    pub fn fliplr(matrix: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_convert_to_vec_array_and_do(matrix, |matrix_as_vec| {
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
        if k > 1 {
            match rot90_once(matrix) {
                Ok(mut mat) => rot90_ktimes(&mut mat, k - 1),
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
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "mtimes", return_raw)]
    pub fn mtimes(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrices_and_compatible_convert_to_vec_array_and_do(
            FOIL::Inside,
            &mut matrix1.clone(),
            &mut matrix2.clone(),
            |matrix_as_vec1, matrix_as_vec2| {
                let dm1 =
                    DMatrix::from_fn(matrix_as_vec1.len(), matrix_as_vec1[0].len(), |i, j| {
                        if matrix_as_vec1[0][0].is::<FLOAT>() {
                            matrix_as_vec1[i][j].as_float().unwrap()
                        } else {
                            matrix_as_vec1[i][j].as_int().unwrap() as FLOAT
                        }
                    });

                let dm2 =
                    DMatrix::from_fn(matrix_as_vec2.len(), matrix_as_vec2[0].len(), |i, j| {
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
            },
        )
    }

    /// Concatenate two arrays horizontally.
    /// ```typescript
    /// let arr1 = eye(3);
    /// let arr2 = eye(3);
    /// let combined = horzcat(arr1, arr2);
    /// assert_eq(size(combined), [3, 6]);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "horzcat", return_raw)]
    pub fn horzcat(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrices_and_compatible_convert_to_vec_array_and_do(
            FOIL::First,
            &mut matrix1.clone(),
            &mut matrix2.clone(),
            |matrix_as_vec1, matrix_as_vec2| {
                let dm1 =
                    DMatrix::from_fn(matrix_as_vec1.len(), matrix_as_vec1[0].len(), |i, j| {
                        if matrix_as_vec1[0][0].is::<FLOAT>() {
                            matrix_as_vec1[i][j].as_float().unwrap()
                        } else {
                            matrix_as_vec1[i][j].as_int().unwrap() as FLOAT
                        }
                    });

                let dm2 =
                    DMatrix::from_fn(matrix_as_vec2.len(), matrix_as_vec2[0].len(), |i, j| {
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
            },
        )
    }

    /// Concatenates two array vertically.
    /// ```typescript
    /// let arr1 = eye(3);
    /// let arr2 = eye(3);
    /// let combined = vertcat(arr1, arr2);
    /// assert_eq(size(combined), [6, 3]);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "vertcat", return_raw)]
    pub fn vertcat(matrix1: Array, matrix2: Array) -> Result<Array, Box<EvalAltResult>> {
        if_matrices_and_compatible_convert_to_vec_array_and_do(
            FOIL::Last,
            &mut matrix1.clone(),
            &mut matrix2.clone(),
            |matrix_as_vec1, matrix_as_vec2| {
                let dm1 =
                    DMatrix::from_fn(matrix_as_vec1.len(), matrix_as_vec1[0].len(), |i, j| {
                        if matrix_as_vec1[0][0].is::<FLOAT>() {
                            matrix_as_vec1[i][j].as_float().unwrap()
                        } else {
                            matrix_as_vec1[i][j].as_int().unwrap() as FLOAT
                        }
                    });

                let dm2 =
                    DMatrix::from_fn(matrix_as_vec2.len(), matrix_as_vec2[0].len(), |i, j| {
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
            },
        )
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
    /// let matrix = eye(3);
    /// let combined = repmat(matrix, 2, 2);
    /// assert_eq(size(combined), [6, 6]);
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "repmat", return_raw)]
    pub fn repmat(matrix: &mut Array, nx: INT, ny: INT) -> Result<Array, Box<EvalAltResult>> {
        if_matrix_do(matrix, |matrix| {
            let mut row_matrix = matrix.clone();
            for _ in 1..ny {
                match horzcat(row_matrix, matrix.clone()) {
                    Ok(mat) => row_matrix = mat,
                    Err(e) => return Err(e),
                };
            }
            let mut new_matrix = row_matrix.clone();
            for _ in 1..nx {
                match vertcat(new_matrix, row_matrix.clone()) {
                    Ok(mat) => new_matrix = mat,
                    Err(e) => return Err(e),
                }
            }
            Ok(new_matrix)
        })
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
        if_list_do(&mut x.clone(), |x| {
            if_list_do(&mut y.clone(), |y| {
                let nx = x.len();
                let ny = y.len();
                let x_dyn: Vec<Dynamic> = vec![Dynamic::from_array(x.to_vec()); nx];
                let mut y_dyn: Vec<Dynamic> = vec![Dynamic::from_array(y.to_vec()); ny];

                let mut result = BTreeMap::new();
                let mut xid = smartstring::SmartString::new();
                xid.push_str("x");
                let mut yid = smartstring::SmartString::new();
                yid.push_str("y");
                result.insert(xid, Dynamic::from_array(x_dyn));
                result.insert(yid, Dynamic::from_array(transpose(&mut y_dyn).unwrap()));
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
