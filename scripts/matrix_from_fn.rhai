/**
Instantiates a matrix from an array of matrix dimensions and a function which takes an array
of indices as input, yielding the elements of the matrix.
```javascript
let matrix1 = matrix_from_fn([2, 2], |x| 0.0);
let matrix2 = zeros(2);
assert_eq(matrix1, matrix2);
```
**/

fn matrix_from_fn(s, function) {
    let matrix = zeros(s);
    matrix_from_fn_internal(matrix, function, [])
}

private fn matrix_from_fn_internal(matrix, function, indices_so_far) {
    if type_of(matrix[0]) == "array" {
        for idx in 0..len(matrix) {
            matrix[idx] = matrix_from_fn_internal(matrix[idx], |x| {function.call(indices_so_far + x)}, indices_so_far.drain(|v, i| i > 0));
        }
    } else {
        for idx in 0..len(matrix) {
            matrix[idx] = function.call(indices_so_far + [idx]);
        }
    }
    matrix
}


