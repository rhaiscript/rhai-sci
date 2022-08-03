use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

pub fn if_list_int_float<FA, FB, T>(
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
