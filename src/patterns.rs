use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

pub fn list_int_float_else<FA, FB, T>(
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
