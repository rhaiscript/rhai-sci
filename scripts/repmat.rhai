/**
Repeats copies of a matrix
```javascript
let matrix = rand(3);
let combined = repmat(matrix, 2, 2);
assert_eq(size(combined), [6, 6]);
```
**/
fn repmat(matrix, nx, ny) {
    let row_matrix = matrix;
    for i in 1..ny {
        row_matrix = horzcat(row_matrix, matrix);
    }
    let new_matrix = row_matrix;
    for i in 1..nx {
        new_matrix = vertcat(new_matrix, row_matrix);
    }
    new_matrix
};