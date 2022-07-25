/**
Returns an array of the unique elements in an array.
```javascript
let data = [1, 2, 2, 2, 5, 4, 4, 2, 5, 8];
let u = unique(data);
assert_eq(u, [1, 2, 4, 5, 8]);
```
**/
fn unique(arr) {
    arr.sort();
    arr.dedup();
    arr
};