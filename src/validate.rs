use rhai::plugin::*;

#[export_module]
pub mod asdf {
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Map, Position, FLOAT, INT};

    #[rhai_fn(name = "is_int_list", pure)]
    pub fn is_int_list(arr: &mut Array) -> bool {
        if crate::matrix_functions::matrix_size_by_reference(arr).len() > 1 {
            false
        } else {
            if arr[0].is::<INT>() {
                true
            } else {
                false
            }
        }
    }

    #[rhai_fn(name = "is_float_list")]
    pub fn is_float_list(arr: &mut Array) -> bool {
        if crate::matrix_functions::matrix_size_by_reference(arr).len() > 1 {
            false
        } else {
            if arr[0].is::<FLOAT>() {
                true
            } else {
                false
            }
        }
    }

    #[rhai_fn(name = "is_int_or_float_list")]
    pub fn is_int_or_float_list(arr: &mut Array) -> bool {
        if crate::matrix_functions::matrix_size_by_reference(arr).len() > 1 {
            false
        } else {
            if arr[0].is::<FLOAT>() || arr[0].is::<INT>() {
                true
            } else {
                false
            }
        }
    }

    #[rhai_fn(name = "is_matrix")]
    pub fn is_matrix(arr: &mut Array) -> bool {
        if crate::matrix_functions::matrix_size_by_reference(arr).len() != 2 {
            false
        } else {
            if crate::stats::prod(crate::matrix_functions::matrix_size_by_reference(arr))
                .unwrap()
                .as_int()
                .unwrap()
                == crate::matrix_functions::numel_by_reference(arr)
            {
                true
            } else {
                false
            }
        }
    }
}
