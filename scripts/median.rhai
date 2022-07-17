/**
Returns the median of a 1-D array.
```javascript
let data = [1, 1, 1, 1, 2, 5, 6, 7, 8];
let m = median(data);
assert_eq(m, 2.0);
```
**/
fn median(arr) {
    prctile(arr, 50);
};