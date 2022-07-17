/**
Returns a given percentile value for a 1-D array of data.
```javascript
let data = [1, 2, 0, 3, 4];
let p = prctile(data, 50);
assert_eq(p, 2.0);
```
**/
fn prctile(arr, p) {
    arr.sort();
    let x = linspace(0, 100, arr.len());
    interp1(x, arr, p)
};