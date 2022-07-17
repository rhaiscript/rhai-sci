/**
Returns the number of elements in an array.
```javascript
let matrix = ones(4, 6);
let n = numel(matrix);
assert_eq(n, 24);
```
**/
fn numel(arr) {
    len(flatten(arr))
};