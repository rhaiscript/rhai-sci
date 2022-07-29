use rhai::plugin::*;

#[export_module]
pub mod stats {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};
    use std::collections::HashMap;

    /// Return the highest value from a pair of numbers. Fails if the numbers are anything other
    /// than INT or FLOAT.
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

    /// Compute the product of an array. Fails if the input is not an array, or if
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
            let mut p = 1.0 as FLOAT;
            for el in arr {
                p *= el.as_float().unwrap()
            }
            Ok(Dynamic::from_float(p))
        } else if arr[0].is::<i64>() {
            let mut p = 1 as INT;
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

    /// Returns the variance of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 3, 4, 5];
    /// let r = rms(data);
    /// assert_eq(r, 3.3166247903554);
    /// ```
    #[rhai_fn(name = "rms", return_raw)]
    pub fn rms(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut sum = 0.0 as FLOAT;

        // Convert if needed
        let mut x: Vec<FLOAT> = if arr[0].is::<INT>() {
            arr.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            arr.iter().map(|el| el.as_float().unwrap()).collect()
        };
        for v in x {
            sum += v.powi(2)
        }
        let d = sum / (arr.len() as FLOAT);
        Ok(Dynamic::from_float(d.sqrt()))
    }

    /// Returns the variance of a 1-D array.
    /// ```typescript
    /// let data = [1, 1, 1, 1, 2, 5, 6, 7, 8];
    /// let m = median(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "median", return_raw)]
    pub fn median(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        // Convert if needed
        let mut x: Vec<FLOAT> = if arr[0].is::<INT>() {
            arr.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            arr.iter().map(|el| el.as_float().unwrap()).collect()
        };

        x.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let med = if x.len() % 2 == 1 {
            x[(x.len() - 1) / 2]
        } else {
            (x[x.len() / 2] + x[x.len() / 2 - 1]) / 2.0
        };

        Ok(Dynamic::from_float(med))
    }

    /// Returns the median absolute deviation of a 1-D array.
    /// ```typescript
    /// let data = [1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.5, 6.0, 6.0, 6.5, 7.0, 7.0, 7.5, 8.0, 9.0, 12.0, 52.0, 90.0];
    /// let m = mad(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "mad", return_raw)]
    pub fn mad(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut m = 0.0 as FLOAT;
        match median(arr.clone()) {
            Ok(raw_median) => m = raw_median.as_float().unwrap(),
            Err(e) => return Err(e),
        }
        // Convert if needed
        let mut x: Vec<FLOAT> = if arr[0].is::<INT>() {
            arr.iter().map(|el| el.as_int().unwrap() as FLOAT).collect()
        } else {
            arr.iter().map(|el| el.as_float().unwrap()).collect()
        };
        let mut dev = vec![];
        for v in x {
            dev.push(Dynamic::from_float((v - m).abs()));
        }
        Ok(median(dev).unwrap())
    }

    /// Returns a given percentile value for a 1-D array of data.
    /// ```typescript
    /// let data = [1, 2, 0, 3, 4];
    /// let p = prctile(data, 50);
    /// assert_eq(p, 2.0);
    /// ```
    #[rhai_fn(name = "prctile", return_raw)]
    pub fn prctile(arr: Array, p: Dynamic) -> Result<FLOAT, Box<EvalAltResult>> {
        let mut float_array = vec![];
        if arr[0].is::<f64>() {
            float_array = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<FLOAT>>();
        } else if arr[0].is::<i64>() {
            float_array = arr
                .iter()
                .map(|el| el.as_int().unwrap() as FLOAT)
                .collect::<Vec<FLOAT>>();
        } else {
            return Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into());
        }

        // Sort
        float_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let sorted_array = float_array
            .iter()
            .map(|el| Dynamic::from_float(*el))
            .collect::<Vec<Dynamic>>();

        let x = crate::matrix_functions::linspace(
            Dynamic::from_int(0),
            Dynamic::from_int(100),
            arr.len() as INT,
        )
        .unwrap();
        crate::misc_functions::interp1(x, sorted_array, p)
    }

    /// Returns the inter-quartile range for a 1-D array.
    /// ```typescript
    /// let data = [1, 1, 1, 1, 1, 1, 1, 5, 6, 9, 9, 9, 9, 9, 9, 9, 9];
    /// let inter_quartile_range = iqr(data);
    /// assert_eq(inter_quartile_range, 8.0);
    /// ```
    #[rhai_fn(name = "iqr", return_raw)]
    pub fn iqr(arr: Array) -> Result<FLOAT, Box<EvalAltResult>> {
        match (
            prctile(arr.clone(), Dynamic::from_int(25)),
            prctile(arr.clone(), Dynamic::from_int(75)),
        ) {
            (Ok(low), Ok(high)) => Ok(high - low),
            (Ok(_), Err(high)) => Err(high),
            (Err(low), Ok(_)) => Err(low),
            (Err(low), Err(_)) => Err(low),
        }
    }

    /// Returns the mode of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 2, 2, 2, 3];
    /// let m = mode(data);
    /// assert_eq(m, 2);
    /// ```
    /// ```typescript
    /// let data = [1.0, 2.0, 2.0, 2.0, 2.0, 3.0];
    /// let m = mode(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "mode", return_raw)]
    pub fn mode(arr: Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if arr[0].is::<FLOAT>() {
            let mut v = arr
                .iter()
                .map(|el| el.as_float().unwrap())
                .collect::<Vec<FLOAT>>();

            let mut counts: HashMap<String, usize> = HashMap::new();

            Ok(Dynamic::from_float(
                v.iter()
                    .copied()
                    .max_by_key(|&n| {
                        let count = counts.entry(format!("{:?}", n)).or_insert(0);
                        *count += 1;
                        *count
                    })
                    .unwrap(),
            ))
        } else if arr[0].is::<INT>() {
            let mut v = arr
                .iter()
                .map(|el| el.as_int().unwrap())
                .collect::<Vec<INT>>();

            let mut counts: HashMap<INT, usize> = HashMap::new();

            Ok(Dynamic::from_int(
                v.iter()
                    .copied()
                    .max_by_key(|&n| {
                        let count = counts.entry(n).or_insert(0);
                        *count += 1;
                        *count
                    })
                    .unwrap(),
            ))
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The elements of the input must either be INT or FLOAT."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Performs ordinary least squares regression.
    /// ```javascript
    /// let x = [[1.0, 0.0], [1.0, 1.0]];
    /// let y = [[0.1],
    ///          [1.0]];
    /// let b = regress(x, y);
    /// assert_eq(b, [[0.1], [0.9]]);
    /// ```
    fn regress(X: Array, Y: Array) -> Result<Array, Box<EvalAltResult>> {
        if crate::validation_functions::is_matrix(&mut X.clone()) {
            if crate::validation_functions::is_column_vector(&mut Y.clone()) {
                let Xt = crate::matrix_functions::transpose(X.clone());
                let A = crate::matrix_functions::mtimes(
                    crate::matrix_functions::mtimes(
                        crate::matrix_functions::invert_matrix(
                            crate::matrix_functions::mtimes(Xt.clone(), X.clone()).unwrap(),
                        )
                        .unwrap(),
                        Xt.clone(),
                    )
                    .unwrap(),
                    Y,
                )
                .unwrap();
                Ok(A)
            } else {
                Err(EvalAltResult::ErrorArithmetic(
                    format!("The second argument must be a column vector."),
                    Position::NONE,
                )
                .into())
            }
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                format!("The first argument must be a matrix."),
                Position::NONE,
            )
            .into())
        }
    }
}
