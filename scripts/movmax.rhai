/**
Returns an array of the moving maximum (with a given width) across the input array.
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = movmax(data, 3);
assert_eq(m, [2, 4, 4, 4, -1, -1, 3, 3, 3, 2]);
```
**/
fn movmax(arr, k) {
    mov(arr, k, "max")
};