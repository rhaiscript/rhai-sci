use rhai::plugin::*;

#[export_module]
pub mod stats {
    use crate::{
        array_to_vec_float, array_to_vec_int, if_list_convert_to_vec_float_and_do, if_list_do,
        if_list_do_int_or_do_float,
    };
    use rhai::{Array, Dynamic, EvalAltResult, Map, Position, FLOAT, INT};
    use std::collections::BTreeMap;
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
        array_max(&mut vec![a, b])
    }

    /// Return the highest value from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let the_highest_number = max([2, 3, 4, 5]);
    /// assert_eq(the_highest_number, 5);
    /// ```
    /// ```typescript
    /// let the_highest_number = max([2, 3.0, 4.12, 5]);
    /// assert_eq(the_highest_number, 5.0);
    /// ```
    #[rhai_fn(name = "max", return_raw)]
    pub fn array_max(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr: &mut Array| {
                let mut y = array_to_vec_int(arr);
                y.sort();
                Ok(Dynamic::from(y[y.len() - 1]))
            },
            |arr: &mut Array| {
                let mut y = array_to_vec_float(arr);
                y.sort_by(|a, b| a.partial_cmp(b).unwrap());
                Ok(Dynamic::from(y[y.len() - 1]))
            },
        )
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
        array_min(&mut vec![a, b])
    }

    /// Return the lowest value from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    ///
    /// ```typescript
    /// let the_lowest_number = min([2, 3, 4, 5]);
    /// assert_eq(the_lowest_number, 2);
    /// ```
    /// ```typescript
    /// let the_lowest_number = min([2, 3.0, 4.12, 5]);
    /// assert_eq(the_lowest_number, 2.0);
    /// ```
    #[rhai_fn(name = "min", return_raw, pure)]
    pub fn array_min(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr: &mut Array| {
                let mut y = array_to_vec_int(arr);
                y.sort();
                Ok(Dynamic::from(y[0]))
            },
            |arr: &mut Array| {
                let mut y = array_to_vec_float(arr);
                y.sort_by(|a, b| a.partial_cmp(b).unwrap());
                Ok(Dynamic::from(y[0]))
            },
        )
    }

    /// Return the highest value from an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let high_and_low = bounds([2, 3, 4, 5]);
    /// assert_eq(high_and_low, [2, 5]);
    /// ```
    #[rhai_fn(name = "bounds", return_raw)]
    pub fn bounds(arr: &mut Array) -> Result<Array, Box<EvalAltResult>> {
        match (array_min(arr), array_max(arr)) {
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
    /// ```typescript
    /// let data = [32, 15, -7.0, 10, 1000, 41.0, 42];
    /// let mk = maxk(data, 3);
    /// assert_eq(mk, [41.0, 42.0, 1000.0]);
    /// ```
    #[rhai_fn(name = "maxk", return_raw, pure)]
    pub fn maxk(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr: &mut Array| {
                let mut y = array_to_vec_int(arr);
                y.sort();
                let r = (y.len() - (k as usize))..(y.len());
                let mut v = Array::new();
                for idx in r {
                    v.push(Dynamic::from(y[idx]));
                }
                Ok(v)
            },
            |arr: &mut Array| {
                let mut y = array_to_vec_float(arr);
                y.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let r = (y.len() - (k as usize))..(y.len());
                let mut v = Array::new();
                for idx in r {
                    v.push(Dynamic::from(y[idx]));
                }
                Ok(v)
            },
        )
    }

    /// Return the `k` lowest values in an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [32, 15, -7, 10, 1000, 41, 42];
    /// let mk = mink(data, 3);
    /// assert_eq(mk, [-7, 10, 15]);
    /// ```
    /// ```typescript
    /// let data = [32, 15.1223232, -7, 10, 1000.00000, 41, 42];
    /// let mk = mink(data, 3);
    /// assert_eq(mk, [-7.0, 10.0, 15.1223232]);
    /// ```
    #[rhai_fn(name = "mink", return_raw, pure)]
    pub fn mink(arr: &mut Array, k: INT) -> Result<Array, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr| {
                let mut y = array_to_vec_int(arr);
                y.sort();
                let r = (0 as usize)..(k as usize);
                let mut v = Array::new();
                for idx in r {
                    v.push(Dynamic::from(y[idx]));
                }
                Ok(v)
            },
            |arr| {
                let mut y = array_to_vec_float(arr);
                y.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let r = (0 as usize)..(k as usize);
                let mut v = Array::new();
                for idx in r {
                    v.push(Dynamic::from(y[idx]));
                }
                Ok(v)
            },
        )
    }

    /// Sum an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = sum(data);
    /// assert_eq(m, 6);
    /// ```
    /// ```typescript
    /// let data = [1, 2.0, 3];
    /// let m = sum(data);
    /// assert_eq(m, 6.0);
    /// ```
    #[rhai_fn(name = "sum", return_raw, pure)]
    pub fn sum(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr| {
                let y = array_to_vec_int(arr);
                Ok(Dynamic::from_int(y.iter().sum()))
            },
            |arr| {
                let y = array_to_vec_float(arr);
                Ok(Dynamic::from_float(y.iter().sum()))
            },
        )
    }

    /// Return the average of an array. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = mean(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "mean", return_raw, pure)]
    pub fn mean(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let l = arr.len() as FLOAT;
        if_list_do_int_or_do_float(
            arr,
            |arr: &mut Array| {
                sum(arr).map(|s| Dynamic::from_float(s.as_int().unwrap() as FLOAT / l))
            },
            |arr: &mut Array| sum(arr).map(|s| Dynamic::from_float(s.as_float().unwrap() / l)),
        )
    }

    /// Return the index of the largest array element. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = argmax(data);
    /// assert_eq(m, 2);
    /// ```
    #[rhai_fn(name = "argmax", return_raw, pure)]
    pub fn argmax(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do(arr, |arr| {
            array_max(arr).map(|m| {
                Dynamic::from_int(
                    arr.iter()
                        .position(|r| format!("{r}") == format!("{m}"))
                        .unwrap() as INT,
                )
            })
        })
    }

    /// Return the index of the smallest array element. Fails if the input is not an array, or if
    /// it is an array with elements other than INT or FLOAT.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let m = argmin(data);
    /// assert_eq(m, 0);
    /// ```
    #[rhai_fn(name = "argmin", return_raw, pure)]
    pub fn argmin(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do(arr, |arr| {
            array_min(arr).map(|m| {
                Dynamic::from_int(
                    arr.iter()
                        .position(|r| format!("{r}") == format!("{m}"))
                        .unwrap() as INT,
                )
            })
        })
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
    #[rhai_fn(name = "prod", return_raw, pure)]
    pub fn prod(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr| {
                let mut p = 1 as INT;
                for el in arr {
                    p *= el.as_int().unwrap()
                }
                Ok(Dynamic::from_int(p))
            },
            |arr| {
                let mut p = 1.0 as FLOAT;
                for el in arr {
                    p *= el.as_float().unwrap()
                }
                Ok(Dynamic::from_float(p))
            },
        )
    }

    /// Returns the variance of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let v = variance(data);
    /// assert_eq(v, 1.0);
    /// ```
    #[rhai_fn(name = "variance", return_raw, pure)]
    pub fn variance(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let m = mean(arr).map(|med| med.as_float().unwrap())?;

        if_list_convert_to_vec_float_and_do(arr, |x| {
            let mut sum = 0.0 as FLOAT;

            for v in &x {
                sum += (v - m).powi(2)
            }
            let d = sum / (x.len() as FLOAT - 1.0);
            Ok(Dynamic::from_float(d))
        })
    }

    /// Returns the standard deviation of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 3];
    /// let v = std(data);
    /// assert_eq(v, 1.0);
    /// ```
    #[rhai_fn(name = "std", return_raw, pure)]
    pub fn std(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        variance(arr).map(|v| Dynamic::from_float(v.as_float().unwrap().sqrt()))
    }

    /// Returns the variance of a 1-D array.
    /// ```typescript
    /// let data = [1, 2, 3, 4, 5];
    /// let r = rms(data);
    /// assert_eq(r, 3.3166247903554);
    /// ```
    #[rhai_fn(name = "rms", return_raw, pure)]
    pub fn rms(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_convert_to_vec_float_and_do(arr, |arr| {
            let mut sum = 0.0 as FLOAT;
            for v in &arr {
                sum += v.powi(2)
            }
            let d = sum / (arr.len() as FLOAT);
            Ok(Dynamic::from_float(d.sqrt()))
        })
    }

    /// Returns the variance of a 1-D array.
    /// ```typescript
    /// let data = [1, 1, 1, 1, 2, 5, 6, 7, 8];
    /// let m = median(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "median", return_raw, pure)]
    pub fn median(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_convert_to_vec_float_and_do(arr, |mut x| {
            x.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let med = if x.len() % 2 == 1 {
                x[(x.len() - 1) / 2]
            } else {
                (x[x.len() / 2] + x[x.len() / 2 - 1]) / 2.0
            };

            Ok(Dynamic::from_float(med))
        })
    }

    /// Returns the median absolute deviation of a 1-D array.
    /// ```typescript
    /// let data = [1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.5, 6.0, 6.0, 6.5, 7.0, 7.0, 7.5, 8.0, 9.0, 12.0, 52.0, 90.0];
    /// let m = mad(data);
    /// assert_eq(m, 2.0);
    /// ```
    #[rhai_fn(name = "mad", return_raw, pure)]
    pub fn mad(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        let m = median(arr).map(|med| med.as_float().unwrap())?;

        if_list_convert_to_vec_float_and_do(arr, |x| {
            let mut dev = vec![];
            for v in x {
                dev.push(Dynamic::from_float((v - m).abs()));
            }
            median(&mut dev)
        })
    }

    /// Returns a given percentile value for a 1-D array of data.
    ///
    /// The array must not be empty.
    ///
    /// If the percentile value is <= 0 or >= 100, returns the minimum and maximum values of the array respectively.
    /// ```typescript
    /// let data = [1, 2, 0, 3, 4];
    /// let p = prctile(data, 50);
    /// assert_eq(p, 2.0);
    /// ```
    #[rhai_fn(name = "prctile", return_raw, pure)]
    pub fn prctile(arr: &mut Array, p: Dynamic) -> Result<FLOAT, Box<EvalAltResult>> {
        if arr.is_empty() {
            return Err(EvalAltResult::ErrorArithmetic(
                "Array must not be empty".to_string(),
                Position::NONE,
            )
            .into());
        }
        if !p.is_float() && !p.is_int() {
            return Err(EvalAltResult::ErrorArithmetic(
                "Percentile value must either be INT or FLOAT".to_string(),
                Position::NONE,
            )
            .into());
        }

        if_list_convert_to_vec_float_and_do(arr, move |mut float_array| {
            match float_array.len() {
                0 => unreachable!(),
                1 => return Ok(float_array[0]),
                _ => (),
            }

            // Sort
            float_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let sorted_array = float_array
                .iter()
                .map(|el| Dynamic::from_float(*el))
                .collect::<Array>();

            let mut x = crate::matrix_functions::linspace(
                Dynamic::from_int(0),
                Dynamic::from_int(100),
                float_array.len() as INT,
            )?;
            crate::misc_functions::interp1(&mut x, sorted_array, p.clone())
        })
    }

    /// Returns the inter-quartile range for a 1-D array.
    /// ```typescript
    /// let data = [1, 1, 1, 1, 1, 1, 1, 5, 6, 9, 9, 9, 9, 9, 9, 9, 9];
    /// let inter_quartile_range = iqr(data);
    /// assert_eq(inter_quartile_range, 8.0);
    /// ```
    #[rhai_fn(name = "iqr", return_raw, pure)]
    pub fn iqr(arr: &mut Array) -> Result<FLOAT, Box<EvalAltResult>> {
        match (
            prctile(arr, Dynamic::from_int(25)),
            prctile(arr, Dynamic::from_int(75)),
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
    #[rhai_fn(name = "mode", return_raw, pure)]
    pub fn mode(arr: &mut Array) -> Result<Dynamic, Box<EvalAltResult>> {
        if_list_do_int_or_do_float(
            arr,
            |arr| {
                let v = array_to_vec_int(arr);

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
            },
            |arr| {
                let v = array_to_vec_float(arr);

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
            },
        )
    }

    /// Performs ordinary least squares regression.
    /// ```typescript
    /// let x = [[1.0, 0.0],
    ///          [1.0, 1.0],
    ///          [1.0, 2.0]];
    /// let y = [[0.1],
    ///          [0.8],
    ///          [2.1]];
    /// let b = regress(x, y);
    /// assert_eq(b,  #{"parameters": [5.551115123125783e-16, 1.0000000000000002],
    ///                 "pvalues": [1.0, 0.1091825535092476],
    ///                 "standard_errors": [0.1118033988749896, 0.17320508075688787]});
    /// ```
    #[cfg(feature = "nalgebra")]
    #[rhai_fn(name = "regress", return_raw, pure)]
    pub fn regress(x: &mut Array, y: Array) -> Result<Map, Box<EvalAltResult>> {
        use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
        let x_transposed = crate::matrix_functions::transpose(x)?;
        let mut data: Vec<(String, Vec<f64>)> = vec![];
        let mut vars = vec![];
        for (iter, column) in x_transposed.iter().enumerate() {
            let var_name = format!("x_{iter}");
            vars.push(var_name.clone());
            data.push((
                var_name,
                crate::array_to_vec_float(&mut column.clone().into_array().unwrap()),
            ));
        }
        data.push((
            "y".to_string(),
            crate::array_to_vec_float(&mut crate::matrix_functions::flatten(&mut y.clone())),
        ));

        let regress_data = RegressionDataBuilder::new().build_from(data).unwrap();

        let model = FormulaRegressionBuilder::new()
            .data(&regress_data)
            .data_columns("y", vars)
            .fit()
            .map_err(|e| EvalAltResult::ErrorArithmetic(e.to_string(), Position::NONE))?;

        let parameters = Dynamic::from_array(
            model
                .iter_parameter_pairs()
                .map(|x| Dynamic::from_float(x.1))
                .collect::<Array>(),
        );
        let pvalues = Dynamic::from_array(
            model
                .iter_p_value_pairs()
                .map(|x| Dynamic::from_float(x.1))
                .collect::<Array>(),
        );
        let standard_errors = Dynamic::from_array(
            model
                .iter_se_pairs()
                .map(|x| Dynamic::from_float(x.1))
                .collect::<Array>(),
        );

        let mut result = BTreeMap::new();
        let mut params = smartstring::SmartString::new();
        params.push_str("parameters");
        result.insert(params, parameters);
        let mut pv = smartstring::SmartString::new();
        pv.push_str("pvalues");
        result.insert(pv, pvalues);
        let mut se = smartstring::SmartString::new();
        se.push_str("standard_errors");
        result.insert(se, standard_errors);
        Ok(result)
    }
}
