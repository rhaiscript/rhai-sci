/**
Transposes a matrix (called as a function with the matrix as input).
```javascript
let matrix = [[1, 2],
              [3, 4]];
assert_eq(transpose(matrix), [[1, 3],
                              [2, 4]]);
```
**/
fn transpose(arr) {
    let s = size(arr);
    let new_arr = zeros(s[1], s[0]);
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
**/
fn transpose() {
    let arr = this;
    let s = size(arr);
    let new_arr = zeros(s[1], s[0]);
    for i in 0..len(arr) {
        for j in 0..len(arr[0]) {
            new_arr[j][i] = arr[i][j]
        }
    }
    new_arr
}
