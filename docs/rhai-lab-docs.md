# Functions
 This package provides a large variety of functions to help with scientific computing. Each one of these is written in Rhai itself! The source code is here.
## argmax(arr)

Returns the index of the largest element in a 1-D array.
```rhai
let arr = [43, 42, 500];
let idx = argmax(arr);
assert_eq(idx, 3);
```

