/**
Returns an array of the moving minimum (with a given width) across the input array.
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = movmin(data, 3);
assert_eq(m, [1, 1, -1, -2, -3, -3, -3, -1, 1, 1]);
```
**/
fn movmin(arr, k) {
    mov(arr, k, "min")
}
