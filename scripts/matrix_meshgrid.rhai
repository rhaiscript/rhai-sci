/**
Returns an object map containing 2-D grid coordinates based on the uni-axial coordinates
contained in arguments x and y.
```javascript
let x = [1, 2];
let y = [3, 4];
let g = meshgrid(x, y);
assert_eq(g, #{"x": [[1, 2],
                     [1, 2]],
               "y": [[3, 3],
                     [4, 4]]});
```
**/
fn meshgrid(x, y) {
    let nx = len(x);
    let ny = len(y);
    let X = [];
    let Y = [];
    X.pad(ny, x);
    Y.pad(nx, y);

    #{x: X, y: transpose(Y)}
};