/**
Returns the root mean square of a 1-D array.
```javascript
let data = [1, 2, 3, 4, 5];
let r = rms(data);
assert_eq(r, 3.3166247903554);
```
**/
fn rms(arr) {
    sqrt(1.0*arr.reduce(|sum, v| sum + v*v, 0)/len(arr))
};