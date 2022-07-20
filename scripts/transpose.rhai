/**
Transposes a matrix (called as a function with the matrix as input).
```javascript
let matrix = [[1, 2],
              [3, 4]];
assert_eq(transpose(matrix), [[1, 3],
                              [2, 4]]);
```
```javascript
let row = [1, 2, 3, 4];
let column = transpose(row);
assert_eq(column, [[1],
                   [2],
                   [3],
                   [4]]);
```
**/
fn transpose(arr) {
    let s = size(arr);
    let new_arr = [];
    if s.len == 1 {
        new_arr = zeros(s[0], 1);
        arr = [arr];
    } else {
        new_arr = zeros(s[1], s[0]);
    }
    for i in 0..len(arr) {
        for j in 0..len(arr[0]) {
            new_arr[j][i] = arr[i][j]
        }
    }
    new_arr
}

/**
Transposes a matrix (called as a method of the matrix).
```javascript
let matrix = [[1, 2],
              [3, 4]];
assert_eq(matrix.transpose(), [[1, 3],
                               [2, 4]]);
```
```javascript
let row = [1, 2, 3, 4];
let column = row.transpose();
assert_eq(column, [[1],
                   [2],
                   [3],
                   [4]]);
```
**/
fn transpose() {
transpose(this)
}
