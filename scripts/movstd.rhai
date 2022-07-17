/**
Returns an array of the moving standard deviation (with a given width) across the input array.
```javascript
let data = [1, 2, 3, 4, 5, 6];
let m = movstd(data, 3);
assert_eq(m, [0.7071067811865476, 1.0, 1.0, 1.0, 1.0, 0.7071067811865476]);
```
**/
fn movstd(arr, k) {
    mov(arr, k, "std")
}