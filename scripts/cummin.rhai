/**
Returns an array representing the cumulative minimum of a 1-D array.
```rhai
let arr = [8, 9, 3, 5, 4, 1];
let c = cummin(arr);
assert_eq(c, [8, 8, 3, 3, 3, 1]);
```
**/
fn cummin(arr) {
    let new_arr = [];
    let current_min = arr[0];
    for el in arr {
        current_min = min(current_min, el);
        new_arr.push(current_min);
    }
    new_arr
};