use rhai::plugin::*;

#[export_module]
pub mod asdf {
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Map, Position, FLOAT, INT};

    #[rhai_fn(name = "is_int_list")]
    pub fn is_int_list(arr: Array) -> bool {
        if crate::matrix_functions::matrix_size(arr.clone()).len() > 1 {
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
    pub fn is_float_list(arr: Array) -> bool {
        if crate::matrix_functions::matrix_size(arr.clone()).len() > 1 {
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
    pub fn is_int_or_float_list(arr: Array) -> bool {
        if crate::matrix_functions::matrix_size(arr.clone()).len() > 1 {
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
    pub fn is_matrix(arr: Array) -> bool {
        if crate::matrix_functions::matrix_size(arr.clone()).len() != 2 {
            false
        } else {
            if crate::stats::prod(crate::matrix_functions::matrix_size(arr.clone()))
                .unwrap()
                .as_int()
                .unwrap()
                == crate::matrix_functions::numel(arr.clone())
            {
                true
            } else {
                false
            }
        }
    }
}
