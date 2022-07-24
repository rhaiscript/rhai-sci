// Test read matrix
let url = "https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv";
// let x = read_matrix(url);
// assert_eq(size(x), [768, 9]);

// Create a matrix of rands
let matrix = rand(3);
assert_eq(size(matrix), [3, 3]);

// Create a matrix of rands
let matrix = rand([3, 3]);
 assert_eq(size(matrix), [3, 3]);

// Create a matrix of rands
let matrix = rand(3, 3);
assert_eq(size(matrix), [3, 3]);

// Test eye
let matrix = eye(3, 3);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);

// Test eye
let matrix = eye(3);
assert_eq(matrix, [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]]);

// Test eye
let matrix = eye([3, 4]);
assert_eq(matrix, [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0]]);

// Test flatten
let matrix = rand(3, 5);
let flat = flatten(matrix);
assert_eq(len(flat), 15);

// Test flatten
let matrix = [[1.0, 2.0, 3.0], [1.0]];
let flat = flatten(matrix);
assert_eq(len(flat), 4);

// Flip left-to-right
let matrix = fliplr([[1.0, 0.0],
                     [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 1.0],
                   [2.0, 0.0]]);

// Flip up-down
let matrix = flipud([[1.0, 0.0],
                     [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 2.0],
                   [1.0, 0.0]]);

// Rotates a matrix 90 degrees counterclockwise.
let matrix = rot90([[1.0, 0.0],
                   [0.0, 2.0]]);
assert_eq(matrix, [[0.0, 2.0],
                  [1.0, 0.0]]);

// Rotates a matrix 90 degrees counterclockwise by `k` increments.
let matrix = rot90([[1.0, 0.0],
                    [0.0, 2.0]], 2);
assert_eq(matrix, [[2.0, 0.0],
                   [0.0, 1.0]]);


// Test mtimes
let a = eye(3);
let b = ones(3);
let c = mtimes(a, b);
assert_eq(b, c);

// Test vertcat
let arr1 = rand(3);
let arr2 = rand(3);
let combined = vertcat(arr1, arr2);
assert_eq(size(combined), [6, 3]);

// Test horzcat
let arr1 = rand(3);
let arr2 = rand(3);
let combined = horzcat(arr1, arr2);
assert_eq(size(combined), [3, 6]);

// Test diag
 let matrix = [[1, 2, 3],
               [4, 5, 6],
               [7, 8, 9]];
 let d = diag(matrix);
 assert_eq(d, [1, 5, 9]);

// Test diag
 let diagonal = [1, 2, 3];
 let matrix = diag(diagonal);
 assert_eq(matrix, [[1.0, 0.0, 0.0],
                    [0.0, 2.0, 0.0],
                    [0.0, 0.0, 3.0]]);