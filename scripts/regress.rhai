/**
Performs ordinary least squares regression.
```javascript
let x = [[1.0, 0.0], [1.0, 1.0]];
let y = [0.1, 1.0].transpose();
let b = regress(x, y);
assert_eq(b, [[0.1], [0.9]]);
```
**/
fn regress(X, Y) {
    let Xt = transpose(X);
    let XtX = matmul(Xt, X);
    let XtXinv = inv(XtX);
    let XtXinvXt = matmul(XtXinv, Xt);
    matmul(XtXinvXt, Y)
};

private fn matmul(a, b) {
    let b2 = transpose(b);

    let c = zeros(a.len, b[0].len);

    for i in 0..c.len {
        for j in 0..c[i].len {
            c[i][j] = 0.0;

            for z in 0..a[i].len {
                c[i][j] += a[i][z] * b2[j][z];
            }
        }
    }

    c
}