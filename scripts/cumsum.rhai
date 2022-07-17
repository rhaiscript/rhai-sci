/**
Returns an array representing the cumulative product of a 1-D array.
```javascript
let arr = [1.1, 2.5, 3.4];
let c = cumsum(arr);
assert_eq(c, [1.1, 3.6, 7.0]);
```
**/
fn cumsum(arr) {
    let new_arr = [];
    let sum = 0;
    for el in arr {
        sum += el;
        new_arr.push(sum);
    }
    new_arr
};