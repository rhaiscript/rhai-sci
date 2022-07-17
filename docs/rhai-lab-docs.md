# Functions
 This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.
## argmax(arr)

Returns the index of the largest element in a 1-D array.
```javascript
let arr = [43, 42, 500];
let idx = argmax(arr);
assert_eq(idx, 2);
```

## argmin(arr)

Returns the index of the smallest element in a 1-D array.
```javascript
let arr = [43, 42, -500];
let idx = argmin(arr);
assert_eq(idx, 2);
```

## assert
### assert(expression)

Assert that a statement is true and throw an error if it is not.
```javascript
assert(2==2);
```

### assert(expression, error)

Assert that a statement is true, and throw a custom error if it is not.
```javascript
assert(2 == 2, "Oh no!");
```

## assert_eq
### assert_eq(a, b)
 Assert that two arguments are equal and throw an error if they are not.
```javascript
assert_eq(2, 2);
```

### assert_eq(a, b, error)

Assert that two arguments are equal, and throw a custom error if it is not.
```javascript
assert_eq(2, 2, "Oh no!");
```

## assert_neq
### assert_neq(a, b)

Assert that two arguments are non-equal and throw an error if they are not.
```javascript
assert_neq(2, 1);
```

### assert_neq(a, b, error)

Assert that two arguments are non-equal, and throw a custom error if it is not.
```javascript
assert_neq(2, 1, "Oh no!");
```

## bounds(arr)

Returns the bounds (smallest and largest elements) of a 1-D array.
```javascript
let arr = [-100, -1, 2, 3, 5, 1000];
let b = bounds(arr);
assert_eq(b, [-100, 1000]);
```

## cummax(arr)

Returns an array representing the cumulative maximum of a 1-D array.
```javascript
let arr = [1, 4, 5, 3, 9, 8];
let c = cummax(arr);
assert_eq(c, [1, 4, 5, 5, 9, 9]);
```

## cummin(arr)

Returns an array representing the cumulative minimum of a 1-D array.
```javascript
let arr = [8, 9, 3, 5, 4, 1];
let c = cummin(arr);
assert_eq(c, [8, 8, 3, 3, 3, 1]);
```

## cumprod(arr)

Returns an array representing the cumulative product of a 1-D array.
```javascript
let arr = [1, 2, 3, 4, 5];
let c = cumprod(arr);
assert_eq(c, [1, 2, 6, 24, 120]);
```

## cumsum(arr)

Returns an array representing the cumulative product of a 1-D array.
```javascript
let arr = [1.1, 2.5, 3.4];
let c = cumsum(arr);
assert_eq(c, [1.1, 3.6, 7.0]);
```

## cumtrapz
### cumtrapz(x, y)

Returns the cumulative approximate integral of the curve defined by Y and x using the trapezoidal method.
```javascript
let y = [1, 2, 3];
let x = [1.0, 1.5, 2.0];
let c = cumtrapz(x, y);
assert_eq(c, [0.0, 0.5, 1.0]);
```

### cumtrapz(y)

Returns the cumulative approximate integral of the curve defined by y, assuming that there is unit spacing.
```javascript
let y = [1, 2, 3];
let c = cumtrapz(y);
assert_eq(c, [0.0, 1.0, 2.0]);
```

## diag(arr)

This function can be used in two distinct ways.
1. If the argument is an 2-D array, `diag` returns an array containing the diagonal of the array.
2. If the argument is a 1-D array, `diag` returns a matrix containing the argument along the
 diagonal and zeros elsewhere.

 ```javascript
 let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
 let d = diag(matrix);
 assert_eq(d, [1, 5, 9]);
 ```
 ```javascript
 let diagonal = [1, 2, 3];
 let matrix = diag(diagonal);
 assert_eq(matrix, [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]);
 ```

## diff(arr)
 Returns the difference between successive elements of a 1-D array.
## eye
### eye(n)
 Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
   either one argument (creating a square matrix) or two arguments (specifying the number of rows
   and columns separately).

### eye(nx, ny)

## fliplr(arr)
 Reverse the rows in a matrix.
## flipud(arr)
 Reverse the columns in a matrix.
## interp1(x, y, xq)
 Given reference data, perform linear interpolation.
## intersect(arr1, arr2)
 Performs set intersection of two arrays
## iqr(arr)
 Returns the inter-quartile range for a 1-D array.
## linspace(x1, x2, n)
 Returns an array containing a number of elements linearly spaced between two bounds.
## logspace(a, b, n)
 Returns an array containing a number of elements logarithmically spaced between two bounds.
## mad(arr)
 Returns the maximum absolute deviation of an arry.
## max
### max(a, b)
 Returns the highest value between a pair of numbers (if called with two arguments) or in a 1-D
   array (if called with a single `Array`-type argument).

### max(arr)

## maxk(arr, k)
 Returns the k highest values from a 1-D array.
## mean(arr)
 Returns the average of a 1-D array.
## median(arr)
 Returns the median of a 1-D array.
## meshgrid(x, y)
 Returns an object map containing 2-D grid coordinates based on the uni-axial coordinates
   contained in arguments x and y.

## min
### min(a, b)
 Returns the lowest value between a pair of numbers (if called with two arguments) or in a 1-D
   array (if called with a single `Array`-type argument).

### min(arr)

## mink(arr, k)
 Returns the k smallest values in a 1-D array.
## mode(arr)
 Returns the mode of a 1-D array.
## mov(arr, k, function_name)
 Applied an operation (given as a function) to the array as a moving window
## movmad(arr, k)
 Returns an array of the moving maximum absolute deviation (with a given width) across the input array.
## movmax(arr, k)
 Returns an array of the moving maximum (with a given width) across the input array.
## movmean(arr, k)
 Returns an array of the moving average (with a given width) across the input array.
## movmedian(arr, k)
 Returns an array of the moving median (with a given width) across the input array.
## movmin(arr, k)
 Returns an array of the moving minimum (with a given width) across the input array.
## movprod(arr, k)
 Returns an array of the moving product (with a given width) across the input array.
## movstd(arr, k)
 Returns an array of the moving standard deviation (with a given width) across the input array.
## movsum(arr, k)
 Returns an array of the moving sum (with a given width) across the input array.
## movvar(arr, k)
 Returns an array of the moving variance (with a given width) across the input array.
## ndims(arr)
 Returns the number of dimensions in an array.
## numel(arr)
 Returns the number of elements in an array.
## ones
### ones(n)

### ones(nx, ny)
 Create an matrix filled with ones. Can be called with either one int argument (creating a square
   matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
   or two arguments (specifying the number of rows and columns separately).

## prctile(arr, p)
 Returns a given percentile value for a 1-D array of data.
## prod(arr)
 The product (multiplication) of all elements in a 1-D array.
## rand
### rand()
 Create a matrix filled with random values between 0 and 1. Can be called with either no argument (returning a single value),
   one int argument (creating a square matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
   or two arguments (specifying the number of rows and columns separately).

### rand(n)

### rand(nx, ny)

## rms(arr)
 Returns the root mean square of a 1-D array.
## rot90
### rot90(mat)
 Rotates a matrix 90 degrees counterclockwise.
### rot90(mat, k)
 Rotates a matrix 90 degrees counterclockwise by `k` increments.
## size(arr)
 Returns the size along each dimension of an array.
## std(arr)
 Returns the standard deviation of an array
## sum
### sum()
 Sums the elements of a 1-D array (called as a method of the array).
### sum(arr)
 Sums the elements of a 1-D array (called as a function with the array as an argument).
## transpose
### transpose()
 Transposes a matrix (called as a method of the matrix).
### transpose(arr)

## trapz
### trapz(x, y)
 Returns the approximate integral of the curve defined by Y and X using the trapezoidal method.
### trapz(y)

## union(arr1, arr2)
 Returns the set union of two ararys.
## unique(arr)
 Returns an array of the unique elements in an array.
## variance(arr)
 Returns the variance of a 1-D array.
## zeros
### zeros(n)

### zeros(nx, ny)
 Create an matrix filled with ones. Can be called with either one int argument (creating a square
    matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
    or two arguments (specifying the number of rows and columns separately).

# Constants
<table><tr><th>Name</th><th>Value</th></tr><tr><td>c</td><td>299792458.0</td></tr><tr><td>e</td><td>2.718281828459045</td></tr><tr><td>g</td><td>9.80665</td></tr><tr><td>pi</td><td>3.141592653589793</td></tr><tr><td>h</td><td>6.626070150000001e-34</td></tr></table>