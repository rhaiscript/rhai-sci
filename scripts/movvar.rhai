/**
Returns an array of the moving variance (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movvar(data, 3);
assert_eq(m, [0.5, 1.0, 1.0, 1.0, 1.0, 0.5]);
```
**/
fn movvar(arr, k) {
    mov(arr, k, "variance")
}