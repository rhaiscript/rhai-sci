/**
Returns an array representing the cumulative product of a 1-D array.
```rhai
let arr = [1, 2, 3, 4, 5];
let c = cumprod(arr);
assert_eq(c, [1, 2, 6, 24, 120]);
```
**/
fn cumprod(arr) {
    let new_arr = [];
    let prod = 1;
    for el in arr {
        prod *= el;
        new_arr.push(prod);
    }
    new_arr
};