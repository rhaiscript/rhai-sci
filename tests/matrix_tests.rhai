// Test matrix inversion 1
let x = [[ 1.0,  0.0,  2.0],
         [-1.0,  5.0,  0.0],
         [ 0.0,  3.0, -9.0]];
let x_inverted = inv(x);
assert_eq(x_inverted, [[0.8823529411764706,  -0.11764705882352941,   0.19607843137254902],
                       [0.17647058823529413,  0.17647058823529413,   0.0392156862745098 ],
                       [0.058823529411764705, 0.058823529411764705, -0.09803921568627451]]
);

// Test matrix inversion 2
let x = [[1, 2],
         [3, 4]];
let x_inverted = inv(x);
assert_eq(x_inverted, [[-2.0, 1.0],
                       [1.5, -0.5]]
);

// Test read matrix
let url = "https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv";
let x = read_matrix(url);
assert_eq(size(x), [768, 9]);

// Transposes a matrix 1
let row = [1, 2, 3, 4];
let column = transpose(row);
assert_eq(column, [[1],
                   [2],
                   [3],
                   [4]]);


// Transposes a matrix 2
let row = [1, 2, 3, 4];
let column = transpose(eye(3));
assert_eq(column, eye(3));
