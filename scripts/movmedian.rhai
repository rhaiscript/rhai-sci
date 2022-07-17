/**
Returns an array of the moving median (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movmedian(data, 3);
assert_eq(m, [1.5, 2.0, 3.0, 4.0, 5.0, 5.5]);
```
**/
fn movmedian(arr, k) {
    mov(arr, k, "median")
}