use rhai::plugin::*;

#[export_module]
pub mod misc_functions {
    use crate::matrix_functions::ndims;
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
        let (ints, floats, total) = crate::validation_functions::int_and_float_totals(arr);
        // Convert if needed
        if ints == total {
            let mut x = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<INT>>();
            x.sort();
            x.dedup();
            Ok(x.iter().map(|el| Dynamic::from_int(*el)).collect())
        } else if floats == total {
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
        // New variables
        let mut new_x = vec![];
        let mut new_y = vec![];
        let mut new_xq = 0.0 as FLOAT;

        // Convert if needed
        if x[0].is::<INT>() {
            new_x = x
                .iter()
                .map(|el| el.as_int().unwrap() as FLOAT)
                .collect::<Vec<FLOAT>>();
        } else if x[0].is::<FLOAT>() {
            new_x = x
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<FLOAT>>();
        } else {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("Elements of array x must be either INT or FLOAT"),
                Position::NONE,
            )
            .into());
        }

        // Convert if needed
        if y[0].is::<INT>() {
            new_y = y
                .iter()
                .map(|el| el.as_int().unwrap() as FLOAT)
                .collect::<Vec<FLOAT>>();
        } else if y[0].is::<FLOAT>() {
            new_y = y
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<FLOAT>>();
        } else {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("Elements of array y must be either INT or FLOAT"),
                Position::NONE,
            )
            .into());
        }

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

        // Identify teh right index
        let mut b: usize = 0;
        for idx in 0..x.len() {
            if new_x[idx] > new_xq {
                b = idx;
                break;
            }
        }
        let a = b - 1;
        Ok(new_y[a] + (new_xq - new_x[a]) * (new_y[b] - new_y[a]) / (new_x[b] - new_x[a]))
    }
}
