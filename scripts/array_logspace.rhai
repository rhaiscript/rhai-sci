/**
Returns an array containing a number of elements logarithmically spaced between two bounds.
```javascript
let x = logspace(1, 3, 3);
assert_eq(x, [10.0, 100.0, 1000.0]);
```
**/
fn logspace(a, b, n) {
    let exponents = linspace(a, b, n);
    exponents.map(|e| 10**e)
};