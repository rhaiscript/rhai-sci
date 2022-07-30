use rhai::plugin::*;

#[export_module]
pub mod cum_functions {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

    /// Returns an array representing the cumulative product of a 1-D array.
    /// ```typescript
    /// let arr = [1, 2, 3, 4, 5];
    /// let c = cumprod(arr);
    /// assert_eq(c, [1, 2, 6, 24, 120]);
    /// ```
    #[rhai_fn(name = "cumprod", return_raw)]
    pub fn cumprod(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<FLOAT>() {
            let mut p = 1.0 as FLOAT;
            let mut y = arr
                .iter()
                .map(|el| {
                    let e = el.as_float().unwrap();
                    p = p * e;
                    Dynamic::from_float(p)
                })
                .collect::<Vec<Dynamic>>();
            Ok(y)
        } else if arr[0].is::<INT>() {
            let mut p = 1 as INT;
            let mut y = arr
                .iter()
                .map(|el| {
                    let e = el.as_int().unwrap();
                    p = p * e;
                    Dynamic::from_int(p)
                })
                .collect::<Vec<Dynamic>>();
            Ok(y)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Returns an array representing the cumulative maximum of a 1-D array.
    /// ```typescript
    /// let arr = [1, 4, 5, 3, 9, 8];
    /// let c = cummax(arr);
    /// assert_eq(c, [1, 4, 5, 5, 9, 9]);
    /// ```
    #[rhai_fn(name = "cummax", return_raw)]
    pub fn cummax(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<FLOAT>() || arr[0].is::<INT>() {
            let mut p = arr[0].clone();
            let mut y = arr
                .iter()
                .map(|el| {
                    p = crate::stats::gen_max(p.clone(), (*el).clone()).unwrap();
                    p.clone()
                })
                .collect::<Vec<Dynamic>>();
            Ok(y)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Returns an array representing the cumulative minimum of a 1-D array.
    /// ```typescript
    /// let arr = [8, 9, 3, 5, 4, 1];
    /// let c = cummin(arr);
    /// assert_eq(c, [8, 8, 3, 3, 3, 1]);
    /// ```
    #[rhai_fn(name = "cummin", return_raw)]
    pub fn cummin(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<FLOAT>() || arr[0].is::<INT>() {
            let mut p = arr[0].clone();
            let mut y = arr
                .iter()
                .map(|el| {
                    p = crate::stats::gen_min(p.clone(), (*el).clone()).unwrap();
                    p.clone()
                })
                .collect::<Vec<Dynamic>>();
            Ok(y)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Returns an array representing the cumulative product of a 1-D array.
    /// ```typescript
    /// let arr = [1.1, 2.5, 3.4];
    /// let c = cumsum(arr);
    /// assert_eq(c, [1.1, 3.6, 7.0]);
    /// ```
    #[rhai_fn(name = "cumsum", return_raw)]
    pub fn cumsum(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<FLOAT>() {
            let mut p = 0.0 as FLOAT;
            let mut y = arr
                .iter()
                .map(|el| {
                    let e = el.as_float().unwrap();
                    p = p + e;
                    Dynamic::from_float(p)
                })
                .collect::<Vec<Dynamic>>();
            Ok(y)
        } else if arr[0].is::<INT>() {
            let mut p = 0 as INT;
            let mut y = arr
                .iter()
                .map(|el| {
                    let e = el.as_int().unwrap();
                    p = p + e;
                    Dynamic::from_int(p)
                })
                .collect::<Vec<Dynamic>>();
            Ok(y)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
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
