/**
Draw a random value between 0 and 1.
```javascript
let r = rand();
assert(r < 1 && r > 0);
```
**/
fn rand() {
    rand_float()
};

/**
Create a matrix filled with random values between 0 and 1. Can be called with one integer argument (creating a square
matrix) or one array argument (indicating the dimensions, such as that which is returned by `size`).
```javascript
let matrix = rand(3);
assert_eq(size(matrix), [3, 3]);
```
```javascript
let matrix = rand([3, 3]);
 assert_eq(size(matrix), [3, 3]);
```
**/
fn rand(n) {
    if type_of(n) == "array" {
        rand(n[0], n[1])
    } else {
        rand(n, n)
    }
};

/**
Create a matrix filled with random values between 0 and 1. Can be called with two arguments (specifying the number of
rows and columns separately).
```javascript
let matrix = rand(3, 3);
assert_eq(size(matrix), [3, 3]);
```
**/
fn rand(nx, ny) {
    let matrix = zeros(nx, ny);
    for i in 0..nx {
        for j in 0..ny {
            matrix[i][j] = rand();
        }
    }
    matrix
};