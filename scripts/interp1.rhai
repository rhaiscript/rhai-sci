/**
Given reference data, perform linear interpolation.
```javascript
let x = [0, 1];
let y = [1, 2];
let xq = 0.5;
let yq = interp1(x, y, xq);
assert_eq(yq, 1.5);
```
**/
fn interp1(x, y, xq) {
    x.sort();
    let b = 0;
    for idx in 0..x.len() {
        if x[idx] > xq {
            b = idx;
            break;
        }
    }
    let a = b - 1;
    y[a] + (xq - x[a])*(y[b] - y[a])/(x[b] - x[a])
};