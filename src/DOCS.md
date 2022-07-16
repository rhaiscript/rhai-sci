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












