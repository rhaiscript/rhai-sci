/**
Returns an array representing the cumulative maximum of a 1-D array.
```javascript
let arr = [1, 4, 5, 3, 9, 8];
let c = cummax(arr);
assert_eq(c, [1, 4, 5, 5, 9, 9]);
```
**/
fn cummax(arr) {
    let new_arr = [];
    let current_max = arr[0];
    for el in arr {
        current_max = max(current_max, el);
        new_arr.push(current_max);
    }
    new_arr
};