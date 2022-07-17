/**
Returns the average of a 1-D array.
```javascript
let data = [1, 2, 3];
let m = mean(data);
assert_eq(m, 2.0);
```
**/
fn mean(arr) {
    1.0*arr.sum()/arr.len()
};