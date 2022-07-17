/**
Returns the inter-quartile range for a 1-D array.
```javascript
let data = [1, 1, 1, 1, 1, 1, 1, 5, 6, 9, 9, 9, 9, 9, 9, 9, 9];
let inter_quartile_range = iqr(data);
assert_eq(inter_quartile_range, 8.0);
```
**/
fn iqr(arr) {
    prctile(arr, 75) - prctile(arr, 25)
};