/**
Reverse the rows in a matrix.
```javascript
let matrix = fliplr([[1.0, 0.0],
                     [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 1.0],
                   [2.0, 0.0]]);
```
**/
fn fliplr(arr) {
    for (row, idx) in arr {
        row.reverse();
        arr[idx] = row;
    }
    arr
};