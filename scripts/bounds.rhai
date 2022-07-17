/**
Returns the bounds (smallest and largest elements) of a 1-D array.
```javascript
let arr = [-100, -1, 2, 3, 5, 1000];
let b = bounds(arr);
assert_eq(b, [-100, 1000]);
```
**/
fn bounds(arr) {
    [min(arr), max(arr)]
};