use rhai::plugin::*;

#[export_module]
pub mod stats {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

    /// Return the highest value from a pair of numbers. Fails if the numbers are anything other
    /// than INT or FLOAT.
    ///
    /// ```typescript
    /// let the_higher_number = max(2, 3);
    /// assert_eq(the_higher_number, 3);
    /// ```
    /// ```typescript
    /// let the_higher_number = max(2.0, 3.0);
    /// assert_eq(the_higher_number, 3.0);
    /// ```
    #[rhai_fn(name = "max", return_raw)]
    pub fn gen_max(a: Dynamic, b: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        array_max(vec![a, b])
    }

    /// Return the highest value from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    ///
    /// ```typescript
    /// let the_highest_number = max([2, 3, 4, 5]);
    /// assert_eq(the_highest_number, 5);
    /// ```
    #[rhai_fn(name = "max", return_raw)]
    pub fn array_max(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Ok(Dynamic::from(y[y.len() - 1]))
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            Ok(Dynamic::from(y[y.len() - 1]))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Return the lowest value from a pair of numbers. Fails if the numbers are anything other
    /// than INT or FLOAT.
    ///
    /// ```typescript
    /// let the_lower_number = min(2, 3);
    /// assert_eq(the_lower_number, 2);
    /// ```
    /// ```typescript
    /// let the_lower_number = min(2.0, 3.0);
    /// assert_eq(the_lower_number, 2.0);
    /// ```
    #[rhai_fn(name = "min", return_raw)]
    pub fn gen_min(a: Dynamic, b: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        array_min(vec![a, b])
    }

    /// Return the lowest value from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    ///
    /// ```typescript
    /// let the_lowest_number = min([2, 3, 4, 5]);
    /// assert_eq(the_lowest_number, 2);
    /// ```
    #[rhai_fn(name = "min", return_raw)]
    pub fn array_min(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Ok(Dynamic::from(y[0]))
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            Ok(Dynamic::from(y[0]))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Return the highest value from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let high_and_low = bounds([2, 3, 4, 5]);
    /// assert_eq(high_and_low, [2, 5]);
    /// ```
    #[rhai_fn(name = "bounds", return_raw)]
    pub fn bounds(arr: Array) -> Result<Array, Box<EvalAltResult>> {
        match (array_min(arr.clone()), array_max(arr.clone())) {
            (Ok(low), Ok(high)) => Ok(vec![low, high]),
            (Ok(_), Err(high)) => Err(high),
            (Err(low), Ok(_)) => Err(low),
            (Err(low), Err(_)) => Err(low),
        }
    }

    /// Returns the `k` highest values from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [32, 15, -7, 10, 1000, 41, 42];
    /// let mk = maxk(data, 3);
    /// assert_eq(mk, [41, 42, 1000]);
    /// ```
    #[rhai_fn(name = "maxk", return_raw)]
    pub fn maxk(arr: Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let r = (y.len() - (k as usize))..(y.len());
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            let r = (y.len() - (k as usize))..(y.len());
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Return the `k` lowest values in an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [32, 15, -7, 10, 1000, 41, 42];
    /// let mk = mink(data, 3);
    /// assert_eq(mk, [-7, 10, 15]);
    /// ```
    #[rhai_fn(name = "mink", return_raw)]
    pub fn mink(arr: Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            y.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let r = (0 as usize)..(k as usize);
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else if arr[0].is::<i64>() {
            let mut y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            y.sort();
            let r = (0 as usize)..(k as usize);
            let mut v = Array::new();
            for idx in r {
                v.push(Dynamic::from(y[idx]));
            }
            Ok(v)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Sum an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = sum(data);
    /// assert_eq(m, 6);
    /// ```
    #[rhai_fn(name = "sum", return_raw)]
    pub fn sum(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let y = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<f64>>();
            Ok(Dynamic::from_float(y.iter().sum()))
        } else if arr[0].is::<i64>() {
            let y = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<i64>>();
            Ok(Dynamic::from_int(y.iter().sum()))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Return the average of an array.Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = mean(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "mean", return_raw)]
    pub fn mean(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let L = arr.len() as f64;
        match sum(arr) {
            Ok(s) => Ok(Dynamic::from_float(if s.is::<f64>() {
                s.as_float().unwrap() / L
            } else {
                (s.as_int().unwrap() as f64) / L
            })),
            Err(e) => Err(e),
        }
    }

    /// Return the index of the largest array element. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = argmax(data);
    /// assert_eq(m, 2);
    /// ```
    #[rhai_fn(name = "argmax", return_raw)]
    pub fn argmax(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let mm = array_max(arr.clone());
        match mm {
            Ok(m) => Ok(Dynamic::from_int(
                arr.iter()
                    .position(|r| {
                        if r.is::<f64>() {
                            r.clone().as_float() == m.clone().as_float()
                        } else {
                            r.clone().as_int() == m.clone().as_int()
                        }
                    })
                    .unwrap() as i64,
            )),
            Err(e) => Err(e),
        }
    }

    /// Return the index of the smallest array element. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = argmin(data);
    /// assert_eq(m, 0);
    /// ```
    #[rhai_fn(name = "argmin", return_raw)]
    pub fn argmin(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let mm = array_min(arr.clone());
        match mm {
            Ok(m) => Ok(Dynamic::from_int(
                arr.iter()
                    .position(|r| {
                        if r.is::<f64>() {
                            r.clone().as_float() == m.clone().as_float()
                        } else {
                            r.clone().as_int() == m.clone().as_int()
                        }
                    })
                    .unwrap() as i64,
            )),
            Err(e) => Err(e),
        }
    }

    /// Compute the produce of an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = prod(data);
    /// assert_eq(m, 6);
    /// ```
    /// ```typescript
    /// let data = [3, 6, 10];
    /// let m = prod(data);
    /// assert_eq(m, 180);
    /// ```
    #[rhai_fn(name = "prod", return_raw)]
    pub fn prod(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<f64>() {
            let mut p = 1.0_f64;
            for el in arr {
                p *= el.as_float().unwrap()
            }
            Ok(Dynamic::from_float(p))
        } else if arr[0].is::<i64>() {
            let mut p = 1_i64;
            for el in arr {
                p *= el.as_int().unwrap()
            }
            Ok(Dynamic::from_int(p))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

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
    #[rhai_fn(name = "trapz", return_raw)]
    pub fn trapz_unit(y: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        // Convert if needed
        let mut Y: Vec<FLOAT> = if y[0].is::<INT>() {
            y.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            y.iter().map(|el| el.as_float().unwrap()).collect()
        };

        let mut trapsum = 0.0;
        for i in 1..y.len() {
            trapsum += (Y[i] + Y[i - 1]) / 2.0;
        }
        Ok(Dynamic::from_float(trapsum))
    }

    /// Returns the variance of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let v = variance(data);
    /// assert_eq(v, 1.0);
    /// ```
    #[rhai_fn(name = "variance", return_raw)]
    pub fn variance(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut m = 0.0 as FLOAT;
        match mean(arr.clone()) {
            Ok(raw_mean) => m = raw_mean.as_float().unwrap(),
            Err(e) => return Err(e),
        }
        let mut sum = 0.0 as FLOAT;

        // Convert if needed
        let mut x: Vec<FLOAT> = if arr[0].is::<INT>() {
            arr.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            arr.iter().map(|el| el.as_float().unwrap()).collect()
        };
        for v in x {
            sum += (v - m).powi(2)
        }
        let d = sum / (arr.len() as FLOAT - 1.0);
        Ok(Dynamic::from_float(d))
    }

    /// Returns the standard deviation of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let v = std(data);
    /// assert_eq(v, 1.0);
    /// ```
    #[rhai_fn(name = "std", return_raw)]
    pub fn std(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut s = Dynamic::FLOAT_ZERO;
        match variance(arr) {
            Ok(v) => s = v,
            Err(e) => return Err(e),
        }
        Ok(Dynamic::from_float(s.as_float().unwrap().sqrt()))
    }
}
