use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};
#[cfg(feature = "smartcore")]
use smartcorelib::linalg::basic::{
    arrays::Array as scArray, arrays::Array2 as scArray2, matrix::DenseMatrix,
};

/// Matrix compatibility conditions
#[allow(dead_code)]
pub enum FOIL {
    /// Height of first matrix must match height of second matrix
    First,
    /// Height of first matrix must match width of second matrix
    Outside,
    /// Width of first matrix must match height of second matrix
    Inside,
    /// Width of first matrix must match width of second matrix
    Last,
}

pub fn int_and_float_totals(arr: &mut Array) -> (INT, INT, INT) {
    crate::matrix_functions::flatten(arr)
        .iter()
        .fold((0, 0, 0), |(i, f, t), x| {
            if x.is_int() {
                (i + 1, f, t + 1)
            } else if x.is_float() {
                (i, f + 1, t + 1)
            } else {
                (i, f, t + 1)
            }
        })
}

pub fn if_list_do_int_or_do_float<FA, FB, T>(
    arr: &mut Array,
    mut f_int: FA,
    mut f_float: FB,
) -> Result<T, Box<EvalAltResult>>
where
    FA: FnMut(&mut Array) -> Result<T, Box<EvalAltResult>>,
    FB: FnMut(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    let (int, float, total) = int_and_float_totals(arr);
    if int == total {
        f_int(arr)
    } else if float == total {
        f_float(arr)
    } else if float + int == total {
        let mut arr_of_float = arr
            .iter()
            .map(|el| {
                Dynamic::from_float(if el.is_int() {
                    el.as_int().unwrap() as FLOAT
                } else if el.is_float() {
                    el.as_float().unwrap()
                } else {
                    unreachable!("This should never happen!");
                })
            })
            .collect::<Array>();
        f_float(&mut arr_of_float)
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            "The elements of the input array must either be INT or FLOAT".to_string(),
            Position::NONE,
        )
        .into())
    }
}

/// Does a function if the input is a list, otherwise throws an error.
pub fn if_list_do<F, T>(arr: &mut Array, mut f: F) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    crate::validation_functions::is_numeric_list(arr)
        .then(|| f(arr))
        .unwrap_or(Err(EvalAltResult::ErrorArithmetic(
            format!("The elements of the input array must either be INT or FLOAT."),
            Position::NONE,
        )
        .into()))
}

pub fn if_list_convert_to_vec_float_and_do<F, T>(
    arr: &mut Array,
    f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(Vec<FLOAT>) -> Result<T, Box<EvalAltResult>>,
{
    if_list_do_int_or_do_float(
        arr,
        |arr: &mut Array| Ok(arr.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()),
        |arr: &mut Array| Ok(arr.iter().map(|el| el.as_float().unwrap()).collect()),
    )
    .and_then(f)
}

/// If the input is an int, convert to a float and do the function. if the input is a float already,
/// the function is still performed.
pub fn if_int_convert_to_float_and_do<F, T>(x: Dynamic, mut f: F) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(FLOAT) -> Result<T, Box<EvalAltResult>>,
{
    let new_x: FLOAT = if x.is_float() {
        x.as_float().unwrap()
    } else if x.is_int() {
        x.as_int().unwrap() as FLOAT
    } else {
        return Err(EvalAltResult::ErrorArithmetic(
            "The input must either be INT or FLOAT".to_string(),
            Position::NONE,
        )
        .into());
    };
    f(new_x)
}

#[cfg(feature = "nalgebra")]
pub fn if_matrix_do<T, F>(matrix: &mut Array, mut f: F) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    crate::validation_functions::is_matrix(matrix)
        .then(|| f(matrix))
        .unwrap_or(Err(EvalAltResult::ErrorArithmetic(
            format!("The input must be a matrix."),
            Position::NONE,
        )
        .into()))
}

#[cfg(feature = "nalgebra")]
pub fn if_matrices_and_compatible_convert_to_vec_array_and_do<T, F>(
    compatibility_condition: FOIL,
    matrix1: &mut Array,
    matrix2: &mut Array,
    mut f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(Vec<Array>, Vec<Array>) -> Result<T, Box<EvalAltResult>>,
{
    if crate::validation_functions::is_matrix(matrix1) {
        if crate::validation_functions::is_matrix(matrix2) {
            let s1 = crate::matrix_functions::matrix_size_by_reference(matrix1);
            let s2 = crate::matrix_functions::matrix_size_by_reference(matrix2);
            if match compatibility_condition {
                FOIL::First => s1[0].as_int().unwrap() == s2[0].as_int().unwrap(),
                FOIL::Outside => s1[0].as_int().unwrap() == s2[1].as_int().unwrap(),
                FOIL::Inside => s1[1].as_int().unwrap() == s2[0].as_int().unwrap(),
                FOIL::Last => s1[1].as_int().unwrap() == s2[1].as_int().unwrap(),
            } {
                // Turn into Vec<Array>
                let matrix_as_vec1 = matrix1
                    .into_iter()
                    .map(|x| x.clone().into_array().unwrap())
                    .collect::<Vec<Array>>();
                // Turn into Vec<Array>
                let matrix_as_vec2 = matrix2
                    .into_iter()
                    .map(|x| x.clone().into_array().unwrap())
                    .collect::<Vec<Array>>();
                f(matrix_as_vec1, matrix_as_vec2)
            } else {
                Err(EvalAltResult::ErrorArithmetic(
                    "The input matrices are not compatible for this operation".to_string(),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                "The second input must be a matrix".to_string(),
                Position::NONE,
            )
            .into())
        }
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            "The first input must be a matrix".to_string(),
            Position::NONE,
        )
        .into())
    }
}

/// If the input is a
pub fn if_matrix_convert_to_vec_array_and_do<F, T>(
    matrix: &mut Array,
    mut f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(Vec<Array>) -> Result<T, Box<EvalAltResult>>,
{
    let matrix_as_vec = matrix
        .into_iter()
        .map(|x| x.clone().into_array().unwrap())
        .collect::<Vec<Array>>();
    if crate::validation_functions::is_matrix(matrix) {
        f(matrix_as_vec)
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            "The input must be a matrix".to_string(),
            Position::NONE,
        )
        .into())
    }
}

#[cfg(feature = "smartcore")]
pub fn if_matrix_convert_to_dense_matrix_and_do<F, T>(
    matrix: &mut Array,
    mut f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: FnMut(DenseMatrix<FLOAT>) -> Result<T, Box<EvalAltResult>>,
{
    if crate::validation_functions::is_matrix(matrix) {
        let matrix_as_vec = matrix
            .clone()
            .iter()
            .map(|x| {
                x.clone()
                    .into_array()
                    .unwrap()
                    .iter()
                    .map(|y| {
                        if y.is::<FLOAT>() {
                            y.clone().as_float().unwrap()
                        } else {
                            y.clone().as_int().unwrap() as FLOAT
                        }
                    })
                    .collect::<Vec<FLOAT>>()
            })
            .collect::<Vec<Vec<FLOAT>>>();
        f(DenseMatrix::from_2d_vec(&matrix_as_vec))
    } else {
        Err(
            EvalAltResult::ErrorArithmetic(format!("The input must be a matrix."), Position::NONE)
                .into(),
        )
    }
}

pub fn if_int_do_else_if_array_do<FA, FB, T>(
    d: Dynamic,
    mut f_int: FA,
    mut f_array: FB,
) -> Result<T, Box<EvalAltResult>>
where
    FA: FnMut(INT) -> Result<T, Box<EvalAltResult>>,
    FB: FnMut(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    if d.is_int() {
        f_int(d.as_int().unwrap())
    } else if d.is_array() {
        if_list_do(&mut d.into_array().unwrap(), |arr| f_array(arr))
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            "The input must be either an INT or an numeric array".to_string(),
            Position::NONE,
        )
        .into())
    }
}

pub fn array_to_vec_int(arr: &mut Array) -> Vec<INT> {
    arr.iter()
        .map(|el| el.as_int().unwrap())
        .collect::<Vec<INT>>()
}

#[cfg(feature = "smartcore")]
pub fn dense_matrix_to_vec_dynamic(dm: DenseMatrix<FLOAT>) -> Vec<Dynamic> {
    let mut output = vec![];
    for idx in 0..dm.shape().0 {
        output.push(Dynamic::from_array(
            dm.get_row(idx)
                .iterator(0)
                .map(|x| Dynamic::from_float(x.clone()))
                .collect::<Vec<Dynamic>>(),
        ));
    }
    output
}

pub fn array_to_vec_float(arr: &mut Array) -> Vec<FLOAT> {
    arr.into_iter()
        .map(|el| el.as_float().unwrap())
        .collect::<Vec<FLOAT>>()
}

#[cfg(feature = "nalgebra")]
pub fn omatrix_to_vec_dynamic(
    mat: nalgebralib::OMatrix<FLOAT, nalgebralib::Dyn, nalgebralib::Dyn>,
) -> Vec<Dynamic> {
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

#[cfg(feature = "nalgebra")]
pub fn ovector_to_vec_dynamic(mat: nalgebralib::OVector<FLOAT, nalgebralib::Dyn>) -> Vec<Dynamic> {
    let mut out = vec![];
    for idx in 0..mat.shape().0 {
        out.push(Dynamic::from_float(mat[idx]));
    }
    out
}
