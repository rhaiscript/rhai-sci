/**
Returns the approximate integral of the curve defined by `y` and `y` using the trapezoidal method.
```javascript
let y = [1.0, 1.5, 2.0];
let x = [1.0, 2.0, 3.0];
let A = trapz(x, y);
assert_eq(A, 1.0);
```
**/
fn trapz(x, y) {
    let trapsum = 0.0;
    for i in 1..len(x) {
        trapsum +=(y[i] - y[i-1])*(x[i]-x[i-1]);
    }
    trapsum
};

/**
Returns the approximate integral of the curve defined by `y`, using the trapezoidal method and assuming that the points
have unit spacing..
```javascript
let y = [1, 2, 3];
let A = trapz(y);
assert_eq(A, 2.0);
```
**/
fn trapz(y) {
    let x = linspace(1, len(y), len(y));
    trapz(x, y)
};
