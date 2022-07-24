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
