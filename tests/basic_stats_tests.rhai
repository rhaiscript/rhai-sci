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

// Test sum int
let data = [1, 2, 3];
let s = sum(data);
assert_eq(s, 6);

// Test sum float
let data = [1.1, 2.5, 3.4];
let s = sum(data);
assert_eq(s, 7.0);

// Test mean int
let data = [1, 2, 3];
let m = mean(data);
assert_eq(m, 2.0);

// Test mean float
let data = [1.0, 2.0, 3.0];
let m = mean(data);
assert_eq(m, 2.0);

// Test argmin
let arr = [43, 42, -500];
let idx = argmin(arr);
assert_eq(idx, 2);

// Test argmax
let arr = [43, 42, 500];
let idx = argmax(arr);
assert_eq(idx, 2);