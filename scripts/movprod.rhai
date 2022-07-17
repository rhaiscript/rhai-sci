/**
Returns an array of the moving product (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movprod(data, 3);
assert_eq(m, [2, 6, 24, 60, 120, 30]);
```
**/
fn movprod(arr, k) {
    mov(arr, k, "prod")
}
