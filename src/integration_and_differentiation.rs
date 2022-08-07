use rhai::plugin::*;

#[export_module]
pub mod int_and_diff {
    use crate::if_list_convert_to_vec_float_and_do;
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT};

    /// Returns the approximate integral of the curve defined by `y` and `x` using the trapezoidal method.
    /// ```typescript
    /// let y = [1.0, 1.5, 2.0];
    /// let x = [1.0, 2.0, 3.0];
    /// let A = trapz(x, y);
    /// assert_eq(A, 3.0);
    /// ```
    /// ```typescript
    /// let y = [1, 2, 3];
    /// let x = [1, 2, 3];
    /// let A = trapz(x, y);
    /// assert_eq(A, 4.0);
    /// ```
    #[rhai_fn(name = "trapz", return_raw)]
    pub fn trapz(x: Array, y: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if x.len() != y.len() {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The arrays must have the same length"),
                Position::NONE,
            )
            .into())
        } else {
            if_list_convert_to_vec_float_and_do(&mut y.clone(), |yf| {
                if_list_convert_to_vec_float_and_do(&mut x.clone(), |xf| {
                    let mut trapsum = 0.0;
                    for i in 1..x.len() {
                        trapsum += (yf[i] + yf[i - 1]) * (xf[i] - xf[i - 1]) / 2.0;
                    }
                    Ok(Dynamic::from_float(trapsum))
                })
            })
        }
    }

    /// Returns the approximate integral of the curve defined by `y` using the trapezoidal method.
    /// Assumes that x-values have unit spacing.
    /// ```typescript
    /// let y = [1.0, 1.5, 2.0];
    /// let A = trapz(y);
    /// assert_eq(A, 3.0);
    /// ```
    /// ```typescript
    /// let y = [1, 2, 3];
    /// let A = trapz(y);
    /// assert_eq(A, 4.0);
    /// ```
    #[rhai_fn(name = "trapz", return_raw, pure)]
    pub fn trapz_unit(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_convert_to_vec_float_and_do(arr, |y| {
            let mut trapsum = 0.0 as FLOAT;
            for i in 1..y.len() {
                trapsum += (y[i] + y[i - 1]) / 2.0;
            }
            Ok(Dynamic::from_float(trapsum))
        })
    }

    /// Returns the difference between successive elements of a 1-D array.
    /// ```typescript
    /// let arr = [2, 5, 1, 7, 8];
    /// let d = diff(arr);
    /// assert_eq(d, [3, -4, 6, 1]);
    /// ```
    #[rhai_fn(name = "diff", return_raw, pure)]
    pub fn diff(arr: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        crate::if_list_do_int_or_do_float(
            arr,
            |arr| {
                let mut new_arr = vec![];
                for idx in 1..arr.len() {
                    new_arr.push(Dynamic::from_int(
                        arr[idx].as_int().unwrap() - arr[idx - 1].as_int().unwrap(),
                    ));
                }
                Ok(new_arr)
            },
            |arr| {
                let mut new_arr = vec![];
                for idx in 1..arr.len() {
                    new_arr.push(Dynamic::from_float(
                        arr[idx].as_float().unwrap() - arr[idx - 1].as_float().unwrap(),
                    ));
                }
                Ok(new_arr)
            },
        )
    }
}
