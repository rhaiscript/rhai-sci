use rhai::plugin::*;

#[export_module]
pub mod cum_functions {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

    fn accumulate<G>(arr: Array, f: G) -> Result<Array, Box<EvalAltResult>>
    where
        G: Fn(Array) -> Dynamic,
    {
        let mut new_arr = vec![];
        let n = arr.len() as INT;
        for i in 0..n {
            new_arr.push(f(arr.get(0_usize..=(i as usize)).unwrap().to_vec()))
        }
        Ok(new_arr)
    }

    /// Returns an array representing the cumulative product of a 1-D array.
    /// ```typescript
    /// let arr = [1, 2, 3, 4, 5];
    /// let c = cumprod(arr);
    /// assert_eq(c, [1, 2, 6, 24, 120]);
    /// ```
    #[rhai_fn(name = "cumprod", return_raw)]
    pub fn cumprod(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        accumulate(arr, |x| crate::stats::prod(x).unwrap())
    }

    /// Returns an array representing the cumulative maximum of a 1-D array.
    /// ```typescript
    /// let arr = [1, 4, 5, 3, 9, 8];
    /// let c = cummax(arr);
    /// assert_eq(c, [1, 4, 5, 5, 9, 9]);
    /// ```
    #[rhai_fn(name = "cummax", return_raw)]
    pub fn cummax(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        accumulate(arr, |x| crate::stats::array_max(x).unwrap())
    }

    /// Returns an array representing the cumulative minimum of a 1-D array.
    /// ```typescript
    /// let arr = [8, 9, 3, 5, 4, 1];
    /// let c = cummin(arr);
    /// assert_eq(c, [8, 8, 3, 3, 3, 1]);
    /// ```
    #[rhai_fn(name = "cummin", return_raw)]
    pub fn cummin(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        accumulate(arr, |x| crate::stats::array_min(x).unwrap())
    }

    /// Returns an array representing the cumulative product of a 1-D array.
    /// ```typescript
    /// let arr = [1.1, 2.5, 3.4];
    /// let c = cumsum(arr);
    /// assert_eq(c, [1.1, 3.6, 7.0]);
    /// ```
    #[rhai_fn(name = "cumsum", return_raw)]
    pub fn cumsum(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        accumulate(arr, |x| crate::stats::sum(x).unwrap())
    }

    /// Returns the cumulative approximate integral of the curve defined by Y and x using the trapezoidal method.
    /// ```typescript
    /// let y = [1, 2, 3];
    /// let x = [1, 2, 3];
    /// let c = cumtrapz(x, y);
    /// assert_eq(c, [0.0, 1.5, 4.0]);    
    /// ```
    #[rhai_fn(name = "cumtrapz", return_raw)]
    pub fn cumtrapz(x: Array, y: Array) -> Result<Array, Box<EvalAltResult>> {
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
        let mut cumtrapsum = vec![Dynamic::FLOAT_ZERO];
        for i in 1..x.len() {
            trapsum += (Y[i] + Y[i - 1]) * (X[i] - X[i - 1]) / 2.0;
            cumtrapsum.push(Dynamic::from_float(trapsum));
        }
        Ok(cumtrapsum)
    }

    /// Returns the cumulative approximate integral of the curve defined by Y and x using the
    /// trapezoidal method. Assumes unit spacing in the x direction.
    /// ```typescript
    /// let y = [1, 2, 3];
    /// let c = cumtrapz(y);
    /// assert_eq(c, [0.0, 1.5, 4.0]);    
    /// ```
    #[rhai_fn(name = "cumtrapz", return_raw)]
    pub fn cumtrapz_unit(y: Array) -> Result<Array, Box<EvalAltResult>> {
        // Convert if needed
        let mut Y: Vec<FLOAT> = if y[0].is::<INT>() {
            y.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            y.iter().map(|el| el.as_float().unwrap()).collect()
        };

        let mut trapsum = 0.0;
        let mut cumtrapsum = vec![Dynamic::FLOAT_ZERO];
        for i in 1..y.len() {
            trapsum += (Y[i] + Y[i - 1]) / 2.0;
            cumtrapsum.push(Dynamic::from_float(trapsum));
        }
        Ok(cumtrapsum)
    }
}
