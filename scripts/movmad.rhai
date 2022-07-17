/**
Returns an array of the moving maximum absolute deviation (with a given width) across the input array.
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = movmad(data, 3);
assert_eq(m, [0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]);
```
**/
fn movmad(arr, k) {
    mov(arr, k, "mad")
};