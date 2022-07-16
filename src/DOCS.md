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


## `diag`
This function can be used in two distinct ways.
1. If the argument is an 2-D array, `diag` returns an array containing the diagonal of the array.
2. If the argument is a 1-D array, `diag` returns a matrix containing the argument along the
   diagonal and zeros elsewhere.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
diag([[1, 2, 3], [4, 5, 6], [7, 8, 9]]) // => [1, 5, 9]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![1, 5, 9]);
```
```rust
# use rhai::{Array, serde::from_dynamic};
# use rhai_sci::eval;
# let result: Array = eval("
diag([1, 2, 3]) // => [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# assert_eq!(vecresult, vec![vec![1.0, 0.0, 0.0], vec![0.0, 2.0, 0.0], vec![0.0, 0.0, 3.0]]);
```

## `diff`
Returns the difference between successive elements of a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
diff([2, 5, 1, 7, 8]) // => [3, -4, 6, 1]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![3, -4, 6, 1]);
```

## `eye`
Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
either one argument (creating a square matrix) or two arguments (specifying the number of rows
and columns separately).
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
eye(3) // => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# let sum: f64 = vecresult.into_iter().map(|x| x.into_iter().sum()).collect::<Vec<f64>>().into_iter().sum();
# assert_eq!(sum, 3.0);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
eye(3, 3) // => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# let sum: f64 = vecresult.into_iter().map(|x| x.into_iter().sum()).collect::<Vec<f64>>().into_iter().sum();
# assert_eq!(sum, 3.0);
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

## `min`
Returns the lowest value between a pair of numbers (if called with two arguments) or in a 1-D
array (if called with a single `Array`-type argument).
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
min(43, 42) // => 42
# ").unwrap();
# assert_eq!(result, 42);
```
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
min([43, 42, 500]) // => 42
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
Returns the average of a 1-D array.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
mean([1, 2, 3]) // => 2.0
# ").unwrap();
# assert_eq!(result, 2.0);
```

## `median`
Returns the median of a 1-D array.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
median([1, 1, 1, 1, 2, 5, 6, 7, 8]) // => 2.0
# ").unwrap();
# assert_eq!(result, 2.0);
```


## `meshgrid`
Returns an object map containing 2-D grid coordinates based on the uni-axial coordinates
contained in arguments x and y.
```rust
# use rhai::{Map, serde::from_dynamic};
# use rhai_sci::eval;
# let result: Map = eval("
meshgrid([1, 2], [3, 4]) // => #{\"x\": [[1, 2], [1, 2]], \"y\": [[3, 3], [4, 4]]}
# ").unwrap();
```

## `mink`
Returns the k smallest values in a 1-D array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
mink([32, 15, -7, 10, 1000, 41, 42], 3) // => [-7, 10, 15]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![-7, 10, 15]);
```

## `mode`
Returns the mode of a 1-D array.
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
mode([1, 2, 2, 2, 2, 3]) // => 2
# ").unwrap();
# assert_eq!(result, 2);
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

## `ndims`
Returns the number of dimensions in an array.
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
ndims(ones(4, 6)) // => 2
# ").unwrap();
# assert_eq!(result, 2);
```

## `numel`
Returns the number of elements in an array.
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
numel(ones(4, 6)) // => 24
# ").unwrap();
# assert_eq!(result, 24);
```

## `ones`
Create an matrix filled with ones. Can be called with either one int argument (creating a square
matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
or two arguments (specifying the number of rows and columns separately).
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
ones(3) // => [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
# ").unwrap();
# assert_eq!(result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#    ).collect::<Vec<Vec<f64>>>(), vec![vec![1.0; 3]; 3]);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
ones(3, 3) // => [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
# ").unwrap();
# assert_eq!(result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>(), vec![vec![1.0; 3]; 3]);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
let x = zeros(3, 3);
ones(size(x)) // => [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
# ").unwrap();
# assert_eq!(result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>(), vec![vec![1.0; 3]; 3]);
```


## `prctile`
Returns a given percentile value for a 1-D array of data.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
prctile([1, 2, 0, 3, 4], 50) // => 2.0
# ").unwrap();
# assert_eq!(result, 2.0);
```

## `prod`
The product (multiplication) of all elements in a 1-D array.
```rust
# use rhai::INT;
# use rhai_sci::eval;
# let result: INT = eval("
prod([1, 2, 3, 4, 10]) // => 240
# ").unwrap();
# assert_eq!(result, 240);
```


## `rand`
Create a matrix filled with random values between 0 and 1. Can be called with either no argument (returning a single value), 
one int argument (creating a square matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
or two arguments (specifying the number of rows and columns separately).
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
rand() // => 0.44392202188914254
# ").unwrap();
# assert!(result < 1.0 && result > 0.0);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
rand(3) // => [[0.7333405150571339, 0.3597611759299407, 0.8809543481098305], [0.5327545327750203, 0.9185256001032435, 0.7226084132391764], [0.14803039057912748, 0.8924466624235429, 0.40943835774171167]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# let sum: f64 = vecresult.into_iter().map(|x| x.into_iter().sum()).collect::<Vec<f64>>().into_iter().sum();
# assert!(sum < 9.0 && sum > 0.0);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
let x = zeros(3);
rand(size(x)) // => [[0.7333405150571339, 0.3597611759299407, 0.8809543481098305], [0.5327545327750203, 0.9185256001032435, 0.7226084132391764], [0.14803039057912748, 0.8924466624235429, 0.40943835774171167]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# let sum: f64 = vecresult.into_iter().map(|x| x.into_iter().sum()).collect::<Vec<f64>>().into_iter().sum();
# assert!(sum < 9.0 && sum > 0.0);
```

## `rms`
Returns the root mean square of a 1-D array.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
rms([1, 2, 3, 4, 5]) // => 3.31662479036
# ").unwrap();
# assert_eq!(result, 3.3166247903554);
```


## `rot90`
Rotates a matrix 90 degrees counterclockwise, either in a single increments 
(if a single argument is provided), or in `k` increments (if a second argument is provided).
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
rot90(diag([1, 2])) // => [[0.0, 2.0], [1.0, 0.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# assert_eq!(vecresult, vec![vec![0.0, 2.0], vec![1.0, 0.0]]);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# use rhai::serde::from_dynamic;
# let result: Array = eval("
rot90(diag([1, 2]), 2) // => [[2.0, 0.0], [0.0, 1.0]]
# ").unwrap();
# let vecresult = result.into_iter().map(
#         |x| from_dynamic(&x).unwrap()
#     ).collect::<Vec<Vec<f64>>>();
# assert_eq!(vecresult, vec![vec![2.0, 0.0], vec![0.0, 1.0]]);
```


## `size`
Returns the size along each dimension of an array.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
size(ones(3, 5)) // => [3, 5]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![3, 5]);
```
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
size([[[1, 2]]]) // => [1, 1, 2]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![1, 1, 2]);
```

## `std`
Returns the standard deviation of a 1-D array.
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
std([1, 2, 3]) // => 1.0
# ").unwrap();
# assert_eq!(result, 1.0);
```

## `sum`
Sums the elements of a 1-D array. Can be called as a function or as an array method. 
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
sum([1.1, 2.5, 3.4]) // => 7.0
# ").unwrap();
# assert_eq!(result, 7.0);
```
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
[1.1, 2.5, 3.4].sum() // => 7.0
# ").unwrap();
# assert_eq!(result, 7.0);
```

## `trapz`
Returns the approximate integral of the curve defined by Y and X using the trapezoidal method. 
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
trapz([1.0, 1.5, 2.0], [1, 2, 3]); // => 1.0
# ").unwrap();
# assert_eq!(result, 1.0);
```
```rust
# use rhai::FLOAT;
# use rhai_sci::eval;
# let result: FLOAT = eval("
trapz([1, 2, 3]); // => 2.0
# ").unwrap();
# assert_eq!(result, 2.0);
```

## `union`
Returns the set union of two ararys.
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
union([7, 1, 7, 7, 4], [7, 0, 4, 4, 0]) // => [0, 1, 4, 7]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![0, 1, 4, 7]);
```

## `unique`
Returns an array of the unique elements in an array. 
```rust
# use rhai::Array;
# use rhai_sci::eval;
# let result: Array = eval("
unique([1, 2, 2, 2, 5, 4, 4, 2, 5, 8]) // => [1, 2, 4, 5, 8]
# ").unwrap();
# assert_eq!(result.into_iter().map(|x|x.cast::<i64>()).collect::<Vec<i64>>(), vec![1, 2, 4, 5, 8]);
```