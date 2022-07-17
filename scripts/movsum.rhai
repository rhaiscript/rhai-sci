/**
Returns an array of the moving sum (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movsum(data, 3);
assert_eq(m, [3, 6, 9, 12, 15, 11]);
```
**/
fn movsum(arr, k) {
    mov(arr, k, "sum")
};