/**
Returns the number of dimensions in an array.
```javascript
let matrix = ones(4, 6);
let n = ndims(matrix);
assert_eq(n, 2);
```
**/
fn ndims(arr) {
    len(size(arr))
};