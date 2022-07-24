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
        if arr[0].is::<f64>() {
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
        } else if arr[0].is::<i64>() {
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
        if arr[0].is::<f64>() || arr[0].is::<i64>() {
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
        if arr[0].is::<f64>() || arr[0].is::<i64>() {
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
        if arr[0].is::<f64>() {
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
        } else if arr[0].is::<i64>() {
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
}
