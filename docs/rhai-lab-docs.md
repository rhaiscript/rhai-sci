# Functions
 This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.
## `argmax(arr)`

Returns the index of the largest element in a 1-D array.
```javascript
let arr = [43, 42, 500];
let idx = argmax(arr);
assert_eq(idx, 2);
```

## `argmin(arr)`

Returns the index of the smallest element in a 1-D array.
```javascript
let arr = [43, 42, -500];
let idx = argmin(arr);
assert_eq(idx, 2);
```

## `assert`
### `assert(expression)`

Assert that a statement is true and throw an error if it is not.
```javascript
assert(2==2);
```

### `assert(expression, error)`

Assert that a statement is true, and throw a custom error if it is not.
```javascript
assert(2 == 2, "Oh no!");
```

## `assert_eq`
### `assert_eq(a, b)`
 Assert that two arguments are equal and throw an error if they are not.
```javascript
assert_eq(2, 2);
```

### `assert_eq(a, b, error)`

Assert that two arguments are equal, and throw a custom error if it is not.
```javascript
assert_eq(2, 2, "Oh no!");
```

## `assert_neq`
### `assert_neq(a, b)`

Assert that two arguments are non-equal and throw an error if they are not.
```javascript
assert_neq(2, 1);
```

### `assert_neq(a, b, error)`

Assert that two arguments are non-equal, and throw a custom error if it is not.
```javascript
assert_neq(2, 1, "Oh no!");
```

## `bounds(arr)`

Returns the bounds (smallest and largest elements) of a 1-D array.
```javascript
let arr = [-100, -1, 2, 3, 5, 1000];
let b = bounds(arr);
assert_eq(b, [-100, 1000]);
```

## `cummax(arr)`

Returns an array representing the cumulative maximum of a 1-D array.
```javascript
let arr = [1, 4, 5, 3, 9, 8];
let c = cummax(arr);
assert_eq(c, [1, 4, 5, 5, 9, 9]);
```

## `cummin(arr)`

Returns an array representing the cumulative minimum of a 1-D array.
```javascript
let arr = [8, 9, 3, 5, 4, 1];
let c = cummin(arr);
assert_eq(c, [8, 8, 3, 3, 3, 1]);
```

## `cumprod(arr)`

Returns an array representing the cumulative product of a 1-D array.
```javascript
let arr = [1, 2, 3, 4, 5];
let c = cumprod(arr);
assert_eq(c, [1, 2, 6, 24, 120]);
```

## `cumsum(arr)`

Returns an array representing the cumulative product of a 1-D array.
```javascript
let arr = [1.1, 2.5, 3.4];
let c = cumsum(arr);
assert_eq(c, [1.1, 3.6, 7.0]);
```

## `cumtrapz`
### `cumtrapz(x, y)`

Returns the cumulative approximate integral of the curve defined by Y and x using the trapezoidal method.
```javascript
let y = [1, 2, 3];
let x = [1.0, 1.5, 2.0];
let c = cumtrapz(x, y);
assert_eq(c, [0.0, 0.5, 1.0]);
```

### `cumtrapz(y)`

Returns the cumulative approximate integral of the curve defined by y, assuming that there is unit spacing.
```javascript
let y = [1, 2, 3];
let c = cumtrapz(y);
assert_eq(c, [0.0, 1.0, 2.0]);
```

## `diag(arr)`

This function can be used in two distinct ways.
1. If the argument is an 2-D array, `diag` returns an array containing the diagonal of the array.
2. If the argument is a 1-D array, `diag` returns a matrix containing the argument along the
 diagonal and zeros elsewhere.

 ```javascript
 let matrix = [[1, 2, 3],
               [4, 5, 6],
               [7, 8, 9]];
 let d = diag(matrix);
 assert_eq(d, [1, 5, 9]);
 ```
 ```javascript
 let diagonal = [1, 2, 3];
 let matrix = diag(diagonal);
 assert_eq(matrix, [[1.0, 0.0, 0.0],
                    [0.0, 2.0, 0.0],
                    [0.0, 0.0, 3.0]]);
 ```

## `diff(arr)`

Returns the difference between successive elements of a 1-D array.
```javascript
let arr = [2, 5, 1, 7, 8];
let d = diff(arr);
assert_eq(d, [3, -4, 6, 1]);
```

## `eye`
### `eye(n)`

Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
either one integer argument (creating a square matrix) or with a 2-element array (specifying the number of rows
and columns separately).
```javascript
let matrix = eye(3);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);
```
```javascript
let matrix = eye([3, 3]);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);
```

### `eye(nx, ny)`

Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
two integer arguments (specifying the number of rows and columns separately).
```javascript
let matrix = eye(3, 3);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);
```

## `fliplr(arr)`

Reverse the rows in a matrix.
```javascript
let matrix = fliplr([[1.0, 0.0],
                     [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 1.0],
                   [2.0, 0.0]]);
```

## `flipud(arr)`

Reverse the columns in a matrix.
```javascript
let matrix = flipud([[1.0, 0.0],
                     [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 2.0],
                   [1.0, 0.0]]);
```

## `interp1(x, y, xq)`

Given reference data, perform linear interpolation.
```javascript
let x = [0, 1];
let y = [1, 2];
let xq = 0.5;
let yq = interp1(x, y, xq);
assert_eq(yq, 1.5);
```

## `intersect(arr1, arr2)`

Performs set intersection of two arrays
```javascript
let set1 = [7, 1, 7, 7, 4];
let set2 = [7, 0, 4, 4, 0];
let x = intersect(set1, set2);
assert_eq(x, [4, 7]);
```

## `iqr(arr)`

Returns the inter-quartile range for a 1-D array.
```javascript
let data = [1, 1, 1, 1, 1, 1, 1, 5, 6, 9, 9, 9, 9, 9, 9, 9, 9];
let inter_quartile_range = iqr(data);
assert_eq(inter_quartile_range, 8.0);
```

## `linspace(x1, x2, n)`

Returns an array containing a number of elements linearly spaced between two bounds.
```javascript
let x = linspace(1, 2, 5);
assert_eq(x, [1.0, 1.25, 1.5, 1.75, 2.0]);
```

## `logspace(a, b, n)`

Returns an array containing a number of elements logarithmically spaced between two bounds.
```javascript
let x = logspace(1, 3, 3);
assert_eq(x, [10.0, 100.0, 1000.0]);
```

## `mad(arr)`

Returns the maximum absolute deviation of an array.
```javascript
let data = [1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.5, 6.0, 6.0, 6.5, 7.0, 7.0, 7.5, 8.0, 9.0, 12.0, 52.0, 90.0];
let m = mad(data);
assert_eq(m, 2.0);
```

## `max`
### `max(a, b)`

Returns the highest value from a pair of numbers.
```javascript
let m = max(41, 42);
assert_eq(m, 42);
```

### `max(arr)`

Returns the highest value in an array of numbers.
```javascript
let data = [41, 42, -1, 7, 2];
let m = max(data);
assert_eq(m, 42);
```

## `maxk(arr, k)`

Returns the `k` highest values from an array.
```javascript
let data = [32, 15, -7, 10, 1000, 41, 42];
let mk = maxk(data, 3);
assert_eq(mk, [41, 42, 1000]);
```

## `mean(arr)`

Returns the average of a 1-D array.
```javascript
let data = [1, 2, 3];
let m = mean(data);
assert_eq(m, 2.0);
```

## `median(arr)`

Returns the median of a 1-D array.
```javascript
let data = [1, 1, 1, 1, 2, 5, 6, 7, 8];
let m = median(data);
assert_eq(m, 2.0);
```

## `meshgrid(x, y)`

Returns an object map containing 2-D grid coordinates based on the uni-axial coordinates
contained in arguments x and y.
```javascript
let x = [1, 2];
let y = [3, 4];
let g = meshgrid(x, y);
assert_eq(g, #{"x": [[1, 2],
                     [1, 2]],
               "y": [[3, 3],
                     [4, 4]]});
```

## `min`
### `min(a, b)`

Returns the lowest value from a pair of numbers.
```javascript
let m = min(43, 42);
assert_eq(m, 42);
```

### `min(arr)`

Returns the lowest value in an array of numbers.
```javascript
let data = [41, 42, -1, 7, 2];
let m = min(data);
assert_eq(m, -1);
```

## `mink(arr, k)`

Returns the `k` lowest values from an array.
```javascript
let data = [32, 15, -7, 10, 1000, 41, 42];
let mk = mink(data, 3);
assert_eq(mk, [-7, 10, 15]);
```

## `mode(arr)`

Returns the mode of a 1-D array.
```javascript
let data = [1, 2, 2, 2, 2, 3];
let m = mode(data);
assert_eq(m, 2);
```

## `mov(arr, k, function_name)`

Applied an operation (given as a function) to the array as a moving window
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = mov(data, 3, "mad");
assert_eq(m, [0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]);
```
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = mov(data, 3, "max");
assert_eq(m, [2, 4, 4, 4, -1, -1, 3, 3, 3, 2]);
```

## `movmad(arr, k)`

Returns an array of the moving maximum absolute deviation (with a given width) across the input array.
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = movmad(data, 3);
assert_eq(m, [0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]);
```

## `movmax(arr, k)`

Returns an array of the moving maximum (with a given width) across the input array.
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = movmax(data, 3);
assert_eq(m, [2, 4, 4, 4, -1, -1, 3, 3, 3, 2]);
```

## `movmean(arr, k)`

Returns an array of the moving average (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movmean(data, 3);
assert_eq(m, [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
```

## `movmedian(arr, k)`

Returns an array of the moving median (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movmedian(data, 3);
assert_eq(m, [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
```

## `movmin(arr, k)`

Returns an array of the moving minimum (with a given width) across the input array.
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = movmin(data, 3);
assert_eq(m, [1, 1, -1, -2, -3, -3, -3, -1, 1, 1]);
```

## `movprod(arr, k)`

Returns an array of the moving product (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movprod(data, 3);
assert_eq(m, [2, 6, 24, 60, 120, 30]);
```

## `movstd(arr, k)`

Returns an array of the moving standard deviation (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movstd(data, 3);
assert_eq(m, [0.7071067811865476, 1.0, 1.0, 1.0, 1.0, 0.7071067811865476]);
```

## `movsum(arr, k)`

Returns an array of the moving sum (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movsum(data, 3);
assert_eq(m, [3, 6, 9, 12, 15, 11]);
```

## `movvar(arr, k)`

Returns an array of the moving variance (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movvar(data, 3);
assert_eq(m, [0.5, 1.0, 1.0, 1.0, 1.0, 0.5]);
```

## `ndims(arr)`

Returns the number of dimensions in an array.
```javascript
let matrix = ones(4, 6);
let n = ndims(matrix);
assert_eq(n, 2);
```

## `numel(arr)`

Returns the number of elements in an array.
```javascript
let matrix = ones(4, 6);
let n = numel(matrix);
assert_eq(n, 24);
```

## `ones`
### `ones(n)`

Create an matrix filled with ones. Can be called with either one integer argument (creating a square
matrix) or one array argument (indicating the dimensions, such as that which is returned by `size`).
```javascript
let matrix = ones(3);
assert_eq(matrix, [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]]);
```
```javascript
let matrix = ones([3, 3]);
assert_eq(matrix, [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]]);
```

### `ones(nx, ny)`

Create an matrix filled with ones. Can be called with two integer arguments (specifying the number of rows and columns separately).
```javascript
let matrix = ones(3, 3);
assert_eq(matrix, [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]]);
```

## `prctile(arr, p)`

Returns a given percentile value for a 1-D array of data.
```javascript
let data = [1, 2, 0, 3, 4];
let p = prctile(data, 50);
assert_eq(p, 2.0);
```

## `prod(arr)`

The product (multiplication) of all elements in a 1-D array.
```javascript
let data = [1, 2, 3, 4, 10];
let p = prod(data);
assert_eq(p, 240);
```

## `rand`
### `rand()`

Draw a random value between 0 and 1.
```javascript
let r = rand();
assert(r < 1 && r > 0);
```

### `rand(n)`

Create a matrix filled with random values between 0 and 1. Can be called with one integer argument (creating a square
matrix) or one array argument (indicating the dimensions, such as that which is returned by `size`).
```javascript
let matrix = rand(3);
assert_eq(size(matrix), [3, 3]);
```
```javascript
let matrix = rand([3, 3]);
 assert_eq(size(matrix), [3, 3]);
```

### `rand(nx, ny)`

Create a matrix filled with random values between 0 and 1. Can be called with two arguments (specifying the number of
rows and columns separately).
```javascript
let matrix = rand(3, 3);
assert_eq(size(matrix), [3, 3]);
```

## `rms(arr)`

Returns the root mean square of a 1-D array.
```javascript
let data = [1, 2, 3, 4, 5];
let r = rms(data);
assert_eq(r, 3.3166247903554);
```

## `rot90`
### `rot90(mat)`

Rotates a matrix 90 degrees counterclockwise.
```javascript
let matrix = rot90([[1.0, 0.0],
                   [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 2.0],
                  [1.0, 0.0]]);
```

### `rot90(mat, k)`

Rotates a matrix 90 degrees counterclockwise by `k` increments.
```javascript
let matrix = rot90([[1.0, 0.0],
                    [0.0, 2.0]], 2);
assert_eq(matrix, [[2.0, 0.0],
                   [0.0, 1.0]]);
```

## `size(arr)`

Returns the size along each dimension of an array.
```javascript
let matrix = ones(3, 5);
assert_eq(size(matrix), [3, 5]);
```
```javascript
let matrix = [[[1, 2]]];
assert_eq(size(matrix), [1, 1, 2]);
```

## `std(arr)`
 Returns the standard deviation of an array
```javascript
let data = [1, 2, 3];
let s = std(data);
assert_eq(s, 1.0);
```

## `sum`
### `sum()`

Sums the elements of a 1-D array (called as a method of the array).
```javascript
let s = [1.1, 2.5, 3.4].sum();
assert_eq(s, 7.0);
```

### `sum(arr)`

Sums the elements of a 1-D array (called as a function with the array as an argument).
```javascript
let data = [1.1, 2.5, 3.4];
let s = sum(data);
assert_eq(s, 7.0);
```

## `transpose`
### `transpose()`

Transposes a matrix (called as a method of the matrix).
```javascript
let matrix = [[1, 2],
              [3, 4]];
assert_eq(matrix.transpose(), [[1, 3],
                               [2, 4]]);
```

### `transpose(arr)`

Transposes a matrix (called as a function with the matrix as input).
```javascript
let matrix = [[1, 2],
              [3, 4]];
assert_eq(transpose(matrix), [[1, 3],
                              [2, 4]]);
```

## `trapz`
### `trapz(x, y)`

Returns the approximate integral of the curve defined by `y` and `y` using the trapezoidal method.
```javascript
let y = [1.0, 1.5, 2.0];
let x = [1.0, 2.0, 3.0];
let A = trapz(x, y);
assert_eq(A, 1.0);
```

### `trapz(y)`

Returns the approximate integral of the curve defined by `y`, using the trapezoidal method and assuming that the points
have unit spacing..
```javascript
let y = [1, 2, 3];
let A = trapz(y);
assert_eq(A, 2.0);
```

## `union(arr1, arr2)`

Returns the set union of two ararys.
```javascript
let set1 = [7, 1, 7, 7, 4];
let set2 = [7, 0, 4, 4, 0];
let u = union(set1, set2);
assert_eq(u, [0, 1, 4, 7]);
```

## `unique(arr)`

Returns an array of the unique elements in an array.
```javascript
let data = [1, 2, 2, 2, 5, 4, 4, 2, 5, 8];
let u = unique(data);
assert_eq(u, [1, 2, 4, 5, 8]);
```

## `variance(arr)`

Returns the variance of a 1-D array.
```javascript
let data = [1, 2, 3];
let v = variance(data);
assert_eq(v, 1.0);
```

## `zeros`
### `zeros(n)`

Create an matrix filled with ones. Can be called with either one integer argument (creating a square
matrix) or one array argument (indicating the dimensions, such as that which is returned by `size`)
or two arguments (specifying the number of rows and columns separately).
```javascript
let matrix = zeros(3);
assert_eq(matrix, [[0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0]]);
```
```javascript
let matrix = zeros([3, 3]);
assert_eq(matrix, [[0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0]]);
```

### `zeros(nx, ny)`

Create an matrix filled with ones. Can be called with two arguments (specifying the number of rows and columns separately).
```javascript
let matrix = zeros(3, 3);
assert_eq(matrix, [[0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0]]);
```

# Constants
<table><tr><th>Name</th><th>Value</th></tr><tr><td><code>c<code></td><td><code>299792458.0<code></td></tr><tr><td><code>e<code></td><td><code>2.718281828459045<code></td></tr><tr><td><code>g<code></td><td><code>9.80665<code></td></tr><tr><td><code>pi<code></td><td><code>3.141592653589793<code></td></tr><tr><td><code>h<code></td><td><code>6.626070150000001e-34<code></td></tr></table>