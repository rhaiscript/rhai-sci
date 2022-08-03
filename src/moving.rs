use rhai::plugin::*;

#[export_module]
pub mod moving_functions {
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    fn mov<G>(arr: &mut Array, k: INT, f: G) -> Array
    where
        G: Fn(Array) -> Dynamic,
    {
        // First, validate the inputs
        let mut new_arr = vec![];
        let n = arr.len() as INT;
        for i in 0..n {
            new_arr.push(f(arr
                .get(if k % 2 != 0 {
                    (std::cmp::max(i - (k - 1) / 2, 0) as usize)
                        ..=(std::cmp::min(i + (k - 1) / 2, n - 1) as usize)
                } else {
                    (std::cmp::max(i - k / 2, 0) as usize)
                        ..=(std::cmp::min(i + k / 2 - 1, n - 1) as usize)
                })
                .unwrap()
                .to_vec()))
        }
        new_arr
    }

    /// Returns an array of the moving minimum (with a given width) across the input array.
    /// ```typescript
    /// let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
    /// let m = movmin(data, 3);
    /// assert_eq(m, [1, 1, -1, -2, -3, -3, -3, -1, 1, 1]);
    /// ```
    #[rhai_fn(name = "movmin", return_raw, pure)]
    pub fn movmin(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::array_min(x).unwrap()))
    }

    /// Returns an array of the moving maximum (with a given width) across the input array.
    /// ```typescript
    /// let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
    /// let m = movmax(data, 3);
    /// assert_eq(m, [2, 4, 4, 4, -1, -1, 3, 3, 3, 2]);
    /// ```
    #[rhai_fn(name = "movmax", return_raw, pure)]
    pub fn movmax(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::array_max(x).unwrap()))
    }

    /// Returns an array of the moving maximum absolute deviation (with a given width) across the input array.
    /// ```typescript
    /// let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
    /// let m = movmad(data, 3);
    /// assert_eq(m, [0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]);
    /// ```
    #[rhai_fn(name = "movmad", return_raw, pure)]
    pub fn movmad(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::mad(x).unwrap()))
    }

    /// Returns an array of the moving average (with a given width) across the input array.
    /// ```typescript
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let m = movmean(data, 3);
    /// assert_eq(m, [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
    /// ```
    #[rhai_fn(name = "movmean", return_raw, pure)]
    pub fn movmean(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::mean(x).unwrap()))
    }

    /// Returns an array of the moving median (with a given width) across the input array.
    /// ```typescript
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let m = movmedian(data, 3);
    /// assert_eq(m, [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
    /// ```
    #[rhai_fn(name = "movmedian", return_raw, pure)]
    pub fn movmedian(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::median(x).unwrap()))
    }

    /// Returns an array of the moving product (with a given width) across the input array.
    /// ```javascript
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let m = movprod(data, 3);
    /// assert_eq(m, [2, 6, 24, 60, 120, 30]);
    /// ```
    #[rhai_fn(name = "movprod", return_raw, pure)]
    pub fn movprod(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::prod(x).unwrap()))
    }

    /// Returns an array of the moving standard deviation (with a given width) across the input array.
    /// ```javascript
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let m = movstd(data, 3);
    /// assert_eq(m, [0.7071067811865476, 1.0, 1.0, 1.0, 1.0, 0.7071067811865476]);
    /// ```
    #[rhai_fn(name = "movstd", return_raw, pure)]
    pub fn movstd(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::std(x).unwrap()))
    }

    /// Returns an array of the moving variance (with a given width) across the input array.
    /// ```javascript
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let m = movvar(data, 3);
    /// assert_eq(m, [0.5, 1.0, 1.0, 1.0, 1.0, 0.5]);
    /// ```
    #[rhai_fn(name = "movvar", return_raw, pure)]
    pub fn movvar(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::variance(x).unwrap()))
    }

    /// Returns an array of the moving sum (with a given width) across the input array.
    /// ```javascript
    /// let data = [1, 2, 3, 4, 5, 6];
    /// let m = movsum(data, 3);
    /// assert_eq(m, [3, 6, 9, 12, 15, 11]);
    /// ```
    #[rhai_fn(name = "movsum", return_raw, pure)]
    pub fn movsum(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        Ok(mov(arr, k, |x| crate::stats::sum(x).unwrap()))
    }

    /// Returns an array of the unique elements in an array.
    /// ```typescript
    /// let data = [1, 2, 2, 2, 5, 4, 4, 2, 5, 8];
    /// let u = unique(data);
    /// assert_eq(u, [1, 2, 4, 5, 8]);
    /// ```
    #[rhai_fn(name = "unique", return_raw, pure)]
    pub fn unique(arr: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        // Convert if needed
        if arr[0].is::<INT>() {
            let mut x = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<INT>>();
            x.sort();
            x.dedup();
            Ok(x.iter().map(|el| Dynamic::from_int(*el)).collect())
        } else if arr[0].is::<FLOAT>() {
            let mut x = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<FLOAT>>();
            x.sort_by(|a, b| a.partial_cmp(b).unwrap());
            x.dedup();
            Ok(x.iter().map(|el| Dynamic::from_float(*el)).collect())
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("Elements of array must be either INT or FLOAT"),
                Position::NONE,
            )
            .into())
        }
    }
}
