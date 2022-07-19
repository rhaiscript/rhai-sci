/**
Tries to invert a matrix
```javascript
let x = [[1., 0., 2.], [-1., 5., 0.], [0., 3., -9.]];
assert_eq(2, 2);
```
**/
fn inv(matrix) {
    let new_matrix = matrix;
    if type_of(matrix[0][0]) != "f64" {
        for (xel, idx) in matrix {
            for (yel, jdx) in xel {
                new_matrix[idx][jdx] = 1.0*yel;
            }
        }
    }
    print(new_matrix);
    invert_matrix(new_matrix)
};