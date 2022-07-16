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