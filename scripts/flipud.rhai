/**
Reverse the columns in a matrix.
```javascript
let matrix = flipud([[1.0, 0.0],
                     [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 2.0],
                   [1.0, 0.0]]);
```
**/
fn flipud(arr) {
    transpose(fliplr(transpose(arr)))
};