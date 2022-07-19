/**
Tries to invert a matrix
```javascript
let x = [[ 1.0,  0.0,  2.0],
         [-1.0,  5.0,  0.0],
         [ 0.0,  3.0, -9.0]];
let x_inverted = inv(x);
assert_eq(x_inverted, [[0.8823529411764706,  -0.11764705882352941,   0.19607843137254902],
                       [0.17647058823529413,  0.17647058823529413,   0.0392156862745098 ],
                       [0.058823529411764705, 0.058823529411764705, -0.09803921568627451]]
);
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