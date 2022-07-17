/**
Returns the difference between successive elements of a 1-D array.
```javascript
let arr = [2, 5, 1, 7, 8];
let d = diff(arr);
assert_eq(d, [3, -4, 6, 1]);
```
**/
fn diff(arr) {
    let new_arr = [];
    let sum = 0;
    for idx in 0..(len(arr)-1) {
        new_arr.push(arr[idx+1] - arr[idx]);
    }
    new_arr
};