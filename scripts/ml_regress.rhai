/**
Performs ordinary least squares regression.
```javascript
let x = [[1.0, 0.0], [1.0, 1.0]];
let y = transpose([0.1, 1.0]);
let b = regress(x, y);
assert_eq(b, [[0.1], [0.9]]);
```
**/
fn regress(X, Y) {
    let Xt = transpose(X);
    mtimes(
        mtimes(
            inv(mtimes(Xt, X)),
        Xt),
    Y)
};