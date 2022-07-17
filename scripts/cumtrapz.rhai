/**
Returns the cumulative approximate integral of the curve defined by Y and x using the trapezoidal method.
```javascript
let y = [1, 2, 3];
let x = [1.0, 1.5, 2.0];
let c = cumtrapz(x, y);
assert_eq(c, [0.0, 0.5, 1.0]);
```
**/
fn cumtrapz(x, y) {
    let trapsum = 0.0;
    let cumtrapsum = [0.0];
    for i in 1..len(x) {
        trapsum +=(y[i] - y[i-1])*(x[i]-x[i-1]);
        cumtrapsum.push(trapsum);
    }
    cumtrapsum
}

/**
Returns the cumulative approximate integral of the curve defined by y, assuming that there is unit spacing.
```javascript
let y = [1, 2, 3];
let c = cumtrapz(y);
assert_eq(c, [0.0, 1.0, 2.0]);
```
**/
fn cumtrapz(y) {
    let x = linspace(1, len(y), len(y));
    cumtrapz(x, y)
}

