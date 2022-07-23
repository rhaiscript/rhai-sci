/**
Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
either one integer argument (creating a square matrix) or with a 2-element array (specifying the number of rows
and columns separately).
```javascript
let matrix = eye(3);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);
```
```javascript
let matrix = eye([3, 4]);
assert_eq(matrix, [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0]]);
```
**/
fn eye(n) {
    if type_of(n) == "array" {
        eye(n[0], n[1])
    } else {
        eye(n, n)
    }
};


/**
Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
two integer arguments (specifying the number of rows and columns separately).
```javascript
let matrix = eye(3, 3);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);
```
**/
fn eye(nx, ny) {
    let matrix = zeros(nx, ny);
    for i in 0..min(nx, ny) {
        matrix[i][i] = 1.0;
    }
    matrix
};

