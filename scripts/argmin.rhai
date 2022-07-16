/**
Returns the index of the smallest element in a 1-D array.
```rhai
let arr = [43, 42, -500];
let idx = argmin(arr);
assert_eq(idx, 2);
```
**/
fn argmin(arr) {
    let original = arr;
    arr.sort();
    original.index_of(arr[0])
};