use rhai::plugin::*;

#[export_module]
pub mod int_and_diff {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

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
            return Err(EvalAltResult::ErrorArithmetic(
                format!("The arrays must have the same length"),
                Position::NONE,
            )
            .into());
        }

        // Convert if needed
        let mut X: Vec<FLOAT> = if x[0].is::<INT>() {
            x.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            x.iter().map(|el| el.as_float().unwrap()).collect()
        };

        // Convert if needed
        let mut Y: Vec<FLOAT> = if y[0].is::<INT>() {
            y.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            y.iter().map(|el| el.as_float().unwrap()).collect()
        };

        let mut trapsum = 0.0;
        for i in 1..x.len() {
            trapsum += (Y[i] + Y[i - 1]) * (X[i] - X[i - 1]) / 2.0;
        }
        Ok(Dynamic::from_float(trapsum))
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
    pub fn trapz_unit(y: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        // Convert if needed
        let mut Y: Vec<FLOAT> = if y[0].is::<INT>() {
            y.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            y.iter().map(|el| el.as_float().unwrap()).collect()
        };

        let mut trapsum = 0.0 as FLOAT;
        for i in 1..y.len() {
            trapsum += (Y[i] + Y[i - 1]) / 2.0;
        }
        Ok(Dynamic::from_float(trapsum))
    }

    /// Returns the difference between successive elements of a 1-D array.
    /// ```typescript
    /// let arr = [2, 5, 1, 7, 8];
    /// let d = diff(arr);
    /// assert_eq(d, [3, -4, 6, 1]);
    /// ```
    #[rhai_fn(name = "diff", return_raw, pure)]
    pub fn diff(arr: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<INT>() {
            let mut new_arr = vec![];
            for idx in 1..arr.len() {
                new_arr.push(Dynamic::from_int(
                    arr[idx].as_int().unwrap() - arr[idx - 1].as_int().unwrap(),
                ));
            }
            Ok(new_arr)
        } else if arr[0].is::<FLOAT>() {
            let mut new_arr = vec![];
            for idx in 1..arr.len() {
                new_arr.push(Dynamic::from_float(
                    arr[idx].as_float().unwrap() - arr[idx - 1].as_float().unwrap(),
                ));
            }
            Ok(new_arr)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }
}
