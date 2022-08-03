use rhai::plugin::*;

#[export_module]
pub mod validation_functions {
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Map, Position, FLOAT, INT};

    /// Tests whether the input in a simple list array
    /// ```typescript
    /// let x = [1, 2, 3, 4];
    /// assert_eq(is_list(x), true);
    /// ```
    /// ```typescript
    /// let x = [[[1, 2], [3, 4]]];
    /// assert_eq(is_list(x), false);
    /// ```
    #[rhai_fn(name = "is_list", pure)]
    pub fn is_list(arr: &mut Array) -> bool {
        if crate::matrix_functions::matrix_size_by_reference(arr).len() == 1 {
            true
        } else {
            false
        }
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

    /// Determines if the entire array is numeric (ints or floats).
    /// ```typescript
    /// let x = [1, 2, 3.0, 5.0];
    /// assert_eq(is_numeric_array(x), true);
    /// ```
    /// ```typescript
    /// let x = [1, 2, 3.0, 5.0, "a"];
    /// assert_eq(is_numeric_array(x), false);
    /// ```
    #[rhai_fn(name = "is_numeric_array", pure)]
    pub fn is_numeric_array(arr: &mut Array) -> bool {
        let (ints, floats, total) = int_and_float_totals(arr);
        return if ints + floats - total == 0 {
            true
        } else {
            false
        };
    }

    /// Tests whether the input in a simple list array composed of floating point values.
    /// ```typescript
    /// let x = [1.0, 2.0, 3.0, 4.0];
    /// assert_eq(is_float_list(x), true)
    /// ```
    /// ```typescript
    /// let x = [1, 2, 3, 4];
    /// assert_eq(is_float_list(x), false)
    /// ```
    #[rhai_fn(name = "is_float_list", pure)]
    pub fn is_float_list(arr: &mut Array) -> bool {
        let (_, floats, total) = int_and_float_totals(arr);
        return if (floats == total) && is_list(arr) {
            true
        } else {
            false
        };
    }

    /// Tests whether the input in a simple list array composed of integer values.
    /// ```typescript
    /// let x = [1.0, 2.0, 3.0, 4.0];
    /// assert_eq(is_int_list(x), false)
    /// ```
    /// ```typescript
    /// let x = [1, 2, 3, 4];
    /// assert_eq(is_int_list(x), true)
    /// ```
    #[rhai_fn(name = "is_int_list", pure)]
    pub fn is_int_list(arr: &mut Array) -> bool {
        let (ints, _, total) = int_and_float_totals(arr);
        return if (ints == total) && is_list(arr) {
            true
        } else {
            false
        };
    }

    /// Tests whether the input in a simple list array composed of either floating point or integer values.
    /// ```typescript
    /// let x = [1.0, 2.0, 3.0, 4.0];
    /// assert_eq(is_numeric_list(x), true)
    /// ```
    /// ```typescript
    /// let x = [1, 2, 3, 4];
    /// assert_eq(is_numeric_list(x), true)
    /// ```
    /// ```typescript
    /// let x = ["a", "b", "c", "d"];
    /// assert_eq(is_numeric_list(x), false)
    /// ```
    #[rhai_fn(name = "is_numeric_list", pure)]
    pub fn is_numeric_list(arr: &mut Array) -> bool {
        let (int, float, total) = int_and_float_totals(arr);
        if (int == total || float == total) && is_list(arr) {
            true
        } else {
            false
        }
    }

    /// Tests whether the input is a row vector
    /// ```typescript
    /// let x = ones([1, 5]);
    /// assert_eq(is_row_vector(x), true)
    /// ```
    /// ```typescript
    /// let x = ones([5, 5]);
    /// assert_eq(is_row_vector(x), false)
    /// ```
    #[rhai_fn(name = "is_row_vector", pure)]
    pub fn is_row_vector(arr: &mut Array) -> bool {
        let s = crate::matrix_functions::matrix_size_by_reference(arr);
        if s.len() == 2 && s[0].as_int().unwrap() == 1 {
            true
        } else {
            false
        }
    }

    /// Tests whether the input is a column vector
    /// ```typescript
    /// let x = ones([5, 1]);
    /// assert_eq(is_column_vector(x), true)
    /// ```
    /// ```typescript
    /// let x = ones([5, 5]);
    /// assert_eq(is_column_vector(x), false)
    /// ```
    #[rhai_fn(name = "is_column_vector", pure)]
    pub fn is_column_vector(arr: &mut Array) -> bool {
        let s = crate::matrix_functions::matrix_size_by_reference(arr);
        if s.len() == 2 && s[1].as_int().unwrap() == 1 {
            true
        } else {
            false
        }
    }

    /// Tests whether the input is a matrix
    /// ```typescript
    /// let x = ones([3, 5]);
    /// assert_eq(is_matrix(x), true)
    /// ```
    /// ```typescript
    /// let x = ones([5, 5, 5]);
    /// assert_eq(is_matrix(x), false)
    /// ```
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
