use rhai::plugin::*;

#[export_module]
pub mod misc_functions {
    use crate::if_list_convert_to_vec_float_and_do;
    use crate::if_list_do_int_or_do_float;
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    /// Returns a random number between zero and one.
    /// ```typescript
    /// let r = rand();
    /// assert(r >= 0.0 && r <= 1.0);
    /// ```
    #[rhai_fn(name = "rand")]
    pub fn rand_float() -> FLOAT {
        rand::random()
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
    /// ```typescript
    /// let x = [0, 1];
    /// let y = [1, 2];
    /// let xq = 0.5;
    /// let yq = interp1(x, y, xq);
    /// assert_eq(yq, 1.5);
    /// ```
    #[rhai_fn(name = "interp1", return_raw)]
    pub fn interp1(x: Array, y: Array, xq: Dynamic) -> Result<FLOAT, Box<EvalAltResult>> {
        if_list_convert_to_vec_float_and_do(&mut y.clone(), |new_y| {
            if_list_convert_to_vec_float_and_do(&mut x.clone(), |new_x| {
                let mut new_xq = 0.0 as FLOAT;
                if xq.is::<INT>() {
                    new_xq = xq.as_int().unwrap() as FLOAT;
                } else if xq.is::<FLOAT>() {
                    new_xq = xq.as_float().unwrap();
                } else {
                    return Err(EvalAltResult::ErrorArithmetic(
                        format!("xq must be either INT or FLOAT"),
                        Position::NONE,
                    )
                    .into());
                }

                // Identify the right index
                let mut b: usize = 0;
                for idx in 0..x.len() {
                    if new_x[idx] > new_xq {
                        b = idx;
                        break;
                    }
                }
                let a = b - 1;
                Ok(new_y[a] + (new_xq - new_x[a]) * (new_y[b] - new_y[a]) / (new_x[b] - new_x[a]))
            })
        })
    }
}
