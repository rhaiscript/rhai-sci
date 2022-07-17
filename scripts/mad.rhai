/**
Returns the maximum absolute deviation of an array.
```javascript
let data = [1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.5, 6.0, 6.0, 6.5, 7.0, 7.0, 7.5, 8.0, 9.0, 12.0, 52.0, 90.0];
let m = mad(data);
assert_eq(m, 2.0);
```
**/
fn mad(arr) {
    let m = median(arr);
    median(arr.map(|v| abs(v  - m)))
}