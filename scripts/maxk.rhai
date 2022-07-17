/**
Returns the `k` highest values from an array.
```javascript
let data = [32, 15, -7, 10, 1000, 41, 42];
let mk = maxk(data, 3);
assert_eq(mk, [41, 42, 1000]);
```
**/
fn maxk(arr, k) {
    arr.sort();
    let L = arr.len();
    arr.extract((L-k)..L);
};