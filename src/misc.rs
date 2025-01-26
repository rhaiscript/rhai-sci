use rhai::plugin::*;

#[export_module]
pub mod misc_functions {
    use crate::{if_list_convert_to_vec_float_and_do, if_list_do_int_or_do_float};
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT};

    /// Infinity
    #[allow(non_upper_case_globals)]
    pub const inf: FLOAT = FLOAT::INFINITY;

    /// Returns a random number between zero and one.
    /// ```typescript
    /// let r = rand();
    /// assert(r >= 0.0 && r <= 1.0);
    /// ```
    #[cfg(feature = "rand")]
    #[rhai_fn(name = "rand")]
    pub fn rand_float() -> FLOAT {
        randlib::random()
    }

    /// Returns an array of the unique elements in an array.
    /// ```typescript
    /// let data = [1, 2, 2, 2, 5, 4, 4, 2, 5, 8];
    /// let u = unique(data);
    /// assert_eq(u, [1, 2, 4, 5, 8]);
    /// ```
    #[rhai_fn(name = "unique", return_raw, pure)]
    pub fn unique(arr: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr| {
                let mut x = crate::array_to_vec_int(arr);
                x.sort();
                x.dedup();
                Ok(x.iter().map(|el| Dynamic::from_int(*el)).collect())
            },
            |arr| {
                let mut x = crate::array_to_vec_float(arr);
                x.sort_by(|a, b| a.partial_cmp(b).unwrap());
                x.dedup();
                Ok(x.iter().map(|el| Dynamic::from_float(*el)).collect())
            },
        )
    }

    /// Given reference data, perform linear interpolation.
    ///
    /// Both arrays must be sorted and have the same length.
    ///
    /// Out-of-bound xq values are clamped to the minimum and maximum values of y respectively.
    /// ```typescript
    /// let x = [0, 1];
    /// let y = [1, 2];
    /// let xq = 0.5;
    /// let yq = interp1(x, y, xq);
    /// assert_eq(yq, 1.5);
    /// ```
    #[rhai_fn(name = "interp1", return_raw)]
    pub fn interp1(x: &mut Array, y: Array, xq: Dynamic) -> Result<FLOAT, Box<EvalAltResult>> {
        let new_xq = if xq.is_int() {
            xq.as_int().unwrap() as FLOAT
        } else if xq.is_float() {
            xq.as_float().unwrap()
        } else {
            return Err(EvalAltResult::ErrorArithmetic(
                "xq must be either INT or FLOAT".to_string(),
                Position::NONE,
            )
            .into());
        };

        if x.len() < 2 {
            return Err(EvalAltResult::ErrorArithmetic(
                "The arrays must have at least 2 elements".to_string(),
                Position::NONE,
            )
            .into());
        }
        if x.len() != y.len() {
            return Err(EvalAltResult::ErrorArithmetic(
                "The arrays must have the same length".to_string(),
                Position::NONE,
            )
            .into());
        }

        let mut y = y;

        if_list_convert_to_vec_float_and_do(&mut y, |new_y| {
            if_list_convert_to_vec_float_and_do(x, |new_x| {
                if new_xq >= *new_x.last().unwrap() {
                    return Ok(*new_y.last().unwrap());
                } else if new_xq <= *new_x.first().unwrap() {
                    return Ok(*new_y.first().unwrap());
                }

                // Identify the right index
                let b = new_x
                    .iter()
                    .enumerate()
                    .find_map(|(i, &el)| (el >= new_xq).then(|| i))
                    .unwrap();

                let a = b - 1;
                Ok(new_y[a] + (new_xq - new_x[a]) * (new_y[b] - new_y[a]) / (new_x[b] - new_x[a]))
            })
        })
    }
}
