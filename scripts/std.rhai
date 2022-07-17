/** Returns the standard deviation of an array
```javascript
let data = [1, 2, 3];
let s = std(data);
assert_eq(s, 1.0);
```
**/
fn std(arr) {
    sqrt(variance(arr))
};