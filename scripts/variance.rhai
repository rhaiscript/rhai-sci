/**
Returns the variance of a 1-D array.
```javascript
let data = [1, 2, 3];
let v = variance(data);
assert_eq(v, 1.0);
```
**/
fn variance(arr) {
    let m = mean(arr);
    arr.reduce(|sum, v| sum + (v - m)**2, 0)/(arr.len() - 1)
};