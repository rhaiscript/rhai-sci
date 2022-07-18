/**
Create an matrix filled with ones. Can be called with either one integer argument (creating a square
matrix) or one array argument (indicating the dimensions, such as that which is returned by `size`).
```javascript
let matrix = ones(3);
assert_eq(matrix, [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]]);
```
```javascript
let matrix = ones([3, 3]);
assert_eq(matrix, [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]]);
```
```javascript
let matrix = ones([3, 3, 3]);
assert_eq(matrix, [[[1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0]],
                   [[1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0]],
                   [[1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0]]]);
```
**/
fn ones(n) {
    if type_of(n) == "array" && len(n) > 2 {
        let matrix = [];
        matrix.pad(n[0], ones(n.drain(|v, i| i > 0)));
        matrix
    } else if type_of(n) == "array" && len(n) == 2{
        ones(n[0], n[1])
    } else {
        ones(n, n)
    }
};

/**
Create an matrix filled with ones. Can be called with two integer arguments (specifying the number of rows and columns separately).
```javascript
let matrix = ones(3, 3);
assert_eq(matrix, [[1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0],
                   [1.0, 1.0, 1.0]]);
```
**/

fn ones(nx, ny) {
    let row = [];
    row.pad(ny, 1.0);

    let matrix = [];
    matrix.pad(nx, row);

    matrix
};
