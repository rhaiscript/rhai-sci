// Integer max test
let the_higher_number = max(2, 3);
assert_eq(the_higher_number, 3);

// Float max test
let the_higher_number = max(2.0, 3.0);
assert_eq(the_higher_number, 3.0);

// Array max test
let the_highest_number = max([2, 3, 4, 5]);
assert_eq(the_highest_number, 5);

// Int min test
let the_lower_number = min(2, 3);
assert_eq(the_lower_number, 2);

// Float min test
let the_lower_number = min(2.0, 3.0);
assert_eq(the_lower_number, 2.0);

// Array min test
let the_lowest_number = min([2, 3, 4, 5]);
assert_eq(the_lowest_number, 2);

// Bounds test
let high_and_low = bounds([2, 3, 4, 5]);
assert_eq(high_and_low, [2, 5]);

// Maxk test
let data = [32, 15, -7, 10, 1000, 41, 42];
let mk = maxk(data, 3);
assert_eq(mk, [41, 42, 1000]);

// Mink test
let data = [32, 15, -7, 10, 1000, 41, 42];
let mk = mink(data, 3);
assert_eq(mk, [-7, 10, 15]);

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