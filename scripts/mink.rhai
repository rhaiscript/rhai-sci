/**
Returns the `k` lowest values from an array.
```javascript
let data = [32, 15, -7, 10, 1000, 41, 42];
let mk = mink(data, 3);
assert_eq(mk, [-7, 10, 15]);
```
**/
fn mink(arr, k) {
    arr.sort();
    arr.extract(0..k);
};