use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

pub fn if_list_do_int_or_do_float<FA, FB, T>(
    arr: &mut Array,
    f_int: FA,
    f_float: FB,
) -> Result<T, Box<EvalAltResult>>
where
    FA: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
    FB: Fn(&mut Array) -> Result<T, Box<EvalAltResult>>,
{
    let (int, float, total) = crate::validation_functions::int_and_float_totals(arr);
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

pub fn if_matrix_convert_to_vec_array_and_do<F, T>(
    matrix: &mut Array,
    f: F,
) -> Result<T, Box<EvalAltResult>>
where
    F: Fn(Vec<Array>) -> Result<T, Box<EvalAltResult>>,
{
    {
        let matrix_as_vec = matrix
            .into_iter()
            .map(|x| x.clone().into_array().unwrap())
            .collect::<Vec<Array>>();
        if crate::validation_functions::is_matrix(matrix) {
            f(matrix_as_vec)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The input must be a matrix."),
                Position::NONE,
            )
            .into())
        }
    }
}
