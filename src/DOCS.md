# Functions
This package provides a large variety of functions to help with scientific computing. Each one
of these is written in [`Rhai`](https://rhai.rs/) itself! The source code is [here](https://github.com/cmccomb/rhai-sci/tree/master/scripts).

## `cummax`
Returns an array representing the cumulative sum of a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
cummax([1, 4, 5, 3, 9, 8]) // => [1, 4, 5, 5, 9, 9]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![1, 4, 5, 5, 9, 9]);
```

## `cummin`
Returns an array representing the cumulative sum of a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
cummin([8, 9, 3, 5, 4, 1]) // => [8, 8, 3, 3, 3, 1]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![8, 8, 3, 3, 3, 1]);
```


## `cumsum`
Returns an array representing the cumulative sum of a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
cumsum([1.1, 2.5, 3.4]) // => [1.1, 3.6, 7.0]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![1.1, 3.6, 7.0]);
```

## `cumprod`
Returns an array representing the cumulative product of a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
cumprod([1, 2, 3, 4, 5]) // => [1, 2, 6, 24, 120]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![1, 2, 6, 24, 120]);
```


## `cumtrapz`
Returns the cumulative approximate integral of the curve defined by Y and X using the trapezoidal method.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
cumtrapz([1.0, 1.5, 2.0], [1, 2, 3]); // => [0.0, 0.5, 1.0]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![0.0, 0.5, 1.0]);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
cumtrapz([1, 2, 3]); // => [0.0, 1.0, 2.0]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![0.0, 1.0, 2.0]);
```


## `fiplr`
Reverse the rows in a matrix.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
fliplr(diag([1, 2])) // => [[0.0, 1.0], [2.0, 0.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# assert_eq!(vecresult, vec![vec![0.0, 1.0], vec![2.0, 0.0]]);
```

## `flipud`
Reverse the columns in a matrix.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
flipud(diag([1, 2])) // => [[0.0, 2.0], [1.0, 0.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# assert_eq!(vecresult, vec![vec![0.0, 2.0], vec![1.0, 0.0]]);
```

## `interp1`
Given reference data, perform linear interpolation.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
interp1([0, 1], [1, 2], 0.5) // => 1.5
# ").unwrap();
# assert_eq!(result, 1.5);
```

## `intersect`
Performs set intersection of two arrays
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
intersect([7, 1, 7, 7, 4], [7, 0, 4, 4, 0]) // => [4, 7]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![4, 7]);
```

## `iqr`
Returns the inter-quartile range for a 1-D array.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
iqr([1, 1, 1, 1, 1, 1, 1, 5, 6, 9, 9, 9, 9, 9, 9, 9, 9]) // => 8.0"
# ).unwrap();
# assert_eq!(result, 8.0);
```

## `linspace`
Returns an array containing a number of elements linearly spaced between two bounds.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
linspace(1, 2, 5) // => [1.0, 1.25, 1.5, 1.75, 2.0]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![1.0, 1.25, 1.5, 1.75, 2.0]);
```

## `logspace`
Returns an array containing a number of elements logarithmically spaced between two bounds.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
logspace(1, 3, 3) // => [10.0, 100.0, 1000.0]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![10.0, 100.0, 1000.0]);
```

## `mad`
Returns the maximum absolute deviation of an arry. 
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
mad([1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.5, 6.0, 6.0, 6.5, 7.0, 7.0, 7.5, 8.0, 9.0, 12.0, 52.0, 90.0]) // => 2.0
# ").unwrap();
# assert_eq!(result, 2.0);
```
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
max([41, 42, -1, 7, 2]) // => 42
# ").unwrap();
# assert_eq!(result, 42);
```

## `max`
Returns the highest value between a pair of numbers (if called with two arguments) or in a 1-D
array (if called with a single `Array`-type argument).
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
max(41, 42) // => 42
# ").unwrap();
# assert_eq!(result, 42);
```
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
max([41, 42, -1, 7, 2]) // => 42
# ").unwrap();
# assert_eq!(result, 42);
```

## `maxk`
Returns the k highest values from a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
maxk([32, 15, -7, 10, 1000, 41, 42], 3) // => [41, 42, 1000]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![41, 42, 1000]);
```

## `mean`

```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("

# ").unwrap();
# assert_eq!(result, 2.0);
```






## `movmad`
Returns an array of the moving maximum absolute deviation (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movmad([1, 2, 4, -1, -2, -3, -1, 3, 2, 1], 3) // => [0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]);
```

## `movmax`
Returns an array of the moving maximum (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movmax([1, 2, 4, -1, -2, -3, -1, 3, 2, 1], 3) // => [2, 4, 4, 4, -1, -1, 3, 3, 3, 2]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![2, 4, 4, 4, -1, -1, 3, 3, 3, 2]);
```

## `movmin`
Returns an array of the moving minimum (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movmin([1, 2, 4, -1, -2, -3, -1, 3, 2, 1], 3) // => [1, 1, -1, -2, -3, -3, -3, -1, 1, 1]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![1, 1, -1, -2, -3, -3, -3, -1, 1, 1]);
```

## `movmean`
Returns an array of the moving average (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movmean([1, 2, 3, 4, 5, 6], 3) // => [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
```

## `movmedian`
Returns an array of the moving median (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movmedian([1, 2, 3, 4, 5, 6], 3) // => [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
```

## `movprod`
Returns an array of the moving product (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movprod([1, 2, 3, 4, 5, 6], 3) // => [2, 6, 24, 60, 120, 30]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![2, 6, 24, 60, 120, 30]);
```

## `movstd`
Returns an array of the moving standard deviation (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movstd([1, 2, 3, 4, 5, 6], 3) // => [0.707, 1.0, 1.0, 1.0, 1.0, 0.707]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![0.5_f64.sqrt(), 1.0, 1.0, 1.0, 1.0, 0.5_f64.sqrt()]);
```

## `movsum`
Returns an array of the moving sum (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movsum([1, 2, 3, 4, 5, 6], 3) // => [3, 6, 9, 12, 15, 11]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![3, 6, 9, 12, 15, 11]);
```

## `movvar`
Returns an array of the moving variance (with a given width) across the input array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
movvar([1, 2, 3, 4, 5, 6], 3) // => [0.5, 1.0, 1.0, 1.0, 1.0, 0.5]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<f64>()).collect::<Vec<f64>>(), vec![0.5, 1.0, 1.0, 1.0, 1.0, 0.5]);
```