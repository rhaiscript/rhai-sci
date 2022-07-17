/**
Rotates a matrix 90 degrees counterclockwise.
```javascript
let matrix = rot90([[1.0, 0.0],
                   [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 2.0],
                  [1.0, 0.0]]);
```
**/
fn rot90(mat) {
    let s = size(mat);
    let new_mat = zeros(s);
    for i in 0..s[0] {
        for j in 0..s[1] {
            print(i); print(j);
            new_mat[s[1] - j -1][i] = mat[i][j]
        }
    }
    new_mat
}

/**
Rotates a matrix 90 degrees counterclockwise by `k` increments.
```javascript
let matrix = rot90([[1.0, 0.0],
                    [0.0, 2.0]], 2);
assert_eq(matrix, [[2.0, 0.0],
                   [0.0, 1.0]]);
```
**/
fn rot90(mat, k) {
    for i in 0..k {
        mat = rot90(mat);
    }
    mat
}
