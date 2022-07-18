/**
Create an matrix filled with ones. Can be called with either one integer argument (creating a square
matrix) or one array argument (indicating the dimensions, such as that which is returned by `size`)
or two arguments (specifying the number of rows and columns separately).
```javascript
let matrix = zeros(3);
assert_eq(matrix, [[0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0]]);
```
```javascript
let matrix = zeros([3, 3]);
assert_eq(matrix, [[0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0]]);
```
```javascript
let matrix = zeros([3, 3, 3]);
assert_eq(matrix, [[[0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0]],
                   [[0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0]],
                   [[0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0]]]);
```
**/
fn zeros(n) {
    if type_of(n) == "array" && len(n) > 2 {
        let matrix = [];
        matrix.pad(n[0], zeros(n.drain(|v, i| i > 0)));
        matrix
    } else if type_of(n) == "array" && len(n) == 2 {
        zeros(n[0], n[1])
    } else {
        zeros(n, n)
    }
};


/**
Create an matrix filled with ones. Can be called with two arguments (specifying the number of rows and columns separately).
```javascript
let matrix = zeros(3, 3);
assert_eq(matrix, [[0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0],
                   [0.0, 0.0, 0.0]]);
```
**/
fn zeros(nx, ny) {
    let row = [];
    row.pad(ny, 0.0);

    let matrix = [];
    matrix.pad(nx, row);

    matrix
};