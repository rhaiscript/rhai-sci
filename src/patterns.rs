use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

pub enum FOIL {
    First,
    Outside,
    Inside,
    Last,
}

pub fn int_and_float_totals(arr: &mut Array) -> (INT, INT, INT) {
    crate::matrix_functions::flatten(arr)
        .iter()
        .fold((0, 0, 0), |(i, f, t), x| {
            if x.is::<INT>() {
                (i + 1, f, t + 1)
            } else if x.is::<FLOAT>() {
                (i, f + 1, t + 1)
            } else {
                (i, f, t + 1)
            }
        })
}

pub fn if_list_do_int_or_do_float<FA, FB, T>(
    arr: &mut Array,
    f_int: FA,
    f_float: FB,
) -> Result<T, Box<EvalAltResult>>
where
    FA: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
    FB: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
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
                Dynamic::from_float(if el.is::<INT>() {
                    el.as_int().unwrap() as FLOAT
                } else if el.is::<FLOAT>() {
                    el.as_float().unwrap()
                } else {
                    panic!("This should never happen!");
                })
            })
            .collect::<Vec<Dynamic>>();
        f_float(&mut arr_of_float)
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            format!("The elements of the input array must either be INT or FLOAT."),
            Position::NONE,
        )
        .into())
    }
}

pub fn if_list_do<F, T>(arr: &mut Array, f: F) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    if crate::validation_functions::is_numeric_list(arr) {
        f(arr)
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            format!("The elements of the input array must either be INT or FLOAT."),
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

pub fn array_to_vec_float(arr: &mut Array) -> Vec<FLOAT> {
    arr.iter()
        .map(|el| el.as_float().unwrap())
        .collect::<Vec<FLOAT>>()
}

#[cfg(feature = "matrix")]
pub fn vec_vec_float_to_vec_dynamic(
    mat: nalgebra::OMatrix<FLOAT, nalgebra::Dynamic, nalgebra::Dynamic>,
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

pub fn if_list_convert_to_vec_float_and_do<F, T>(
    arr: &mut Array,
    f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(Vec<FLOAT>) -> Result<T, Box<EvalAltResult>>,
{
    match if_list_do_int_or_do_float(
        arr,
        |arr: &mut Array| Ok(arr.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()),
        |arr: &mut Array| Ok(arr.iter().map(|el| el.as_float().unwrap()).collect()),
    ) {
        Ok(r) => f(r),
        Err(e) => Err(e),
    }
}

pub fn if_int_convert_to_float_and_do<F, T>(x: Dynamic, f: F) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(FLOAT) -> Result<T, Box<EvalAltResult>>,
{
    let new_x: FLOAT = if x.is::<FLOAT>() {
        x.as_float().unwrap()
    } else if x.is::<INT>() {
        x.as_int().unwrap() as FLOAT
    } else {
        return Err(EvalAltResult::ErrorArithmetic(
            format!("The input must either be INT or FLOAT."),
            Position::NONE,
        )
        .into());
    };
    f(new_x)
}

pub fn if_matrix_do<T, F>(matrix: &mut Array, f: F) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    if crate::validation_functions::is_matrix(matrix) {
        f(matrix)
    } else {
        Err(
            EvalAltResult::ErrorArithmetic(format!("The input must be a matrix."), Position::NONE)
                .into(),
        )
    }
}

pub fn if_matrices_and_compatible_convert_to_vec_array_and_do<T, F>(
    compatibility_condition: FOIL,
    matrix1: &mut Array,
    matrix2: &mut Array,
    f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(Vec<Array>, Vec<Array>) -> Result<T, Box<EvalAltResult>>,
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
                // Turn into Vec<Vec<Dynamic>>
                let matrix_as_vec1 = matrix1
                    .into_iter()
                    .map(|x| x.clone().into_array().unwrap())
                    .collect::<Vec<Array>>();
                // Turn into Vec<Vec<Dynamic>>
                let matrix_as_vec2 = matrix2
                    .into_iter()
                    .map(|x| x.clone().into_array().unwrap())
                    .collect::<Vec<Array>>();
                f(matrix_as_vec1, matrix_as_vec2)
            } else {
                Err(EvalAltResult::ErrorArithmetic(
                    format!("The input matrices are not compatible for this operation."),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The second input must be a matrix."),
                Position::NONE,
            )
            .into())
        }
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            format!("The first input must be a matrix."),
            Position::NONE,
        )
        .into())
    }
}

pub fn if_matrix_convert_to_vec_array_and_do<F, T>(
    matrix: &mut Array,
    f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(Vec<Array>) -> Result<T, Box<EvalAltResult>>,
{
    let matrix_as_vec = matrix
        .into_iter()
        .map(|x| x.clone().into_array().unwrap())
        .collect::<Vec<Array>>();
    if crate::validation_functions::is_matrix(matrix) {
        f(matrix_as_vec)
    } else {
        Err(
            EvalAltResult::ErrorArithmetic(format!("The input must be a matrix."), Position::NONE)
                .into(),
        )
    }
}

pub fn if_int_do_else_if_array_do<FA, FB, T>(
    d: Dynamic,
    f_int: FA,
    f_array: FB,
) -> Result<T, Box<EvalAltResult>>
where
    FA: Fn(INT) -> Result<T, Box<EvalAltResult>>,
    FB: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    if d.is::<INT>() {
        f_int(d.as_int().unwrap())
    } else if d.is::<Array>() {
        if_list_do(&mut d.into_array().unwrap(), |arr| f_array(arr))
    } else {
        Err(EvalAltResult::ErrorArithmetic(
            format!("The input must be either an INT or an numeric array."),
            Position::NONE,
        )
        .into())
    }
}
