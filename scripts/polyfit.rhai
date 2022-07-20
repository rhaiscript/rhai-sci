/**
Fits a polynomial of degree n to data given in x and y.
```javascript
let x = [0.0, 1.0];
let y = [0.1, 1.0];
let b = polyfit(x, y, 1);
assert_eq(b, [0.1, 0.9]);
```
```javascript
let x = [0.0, 1.0, 2.0, 3.0];
let y = [0.0, 1.0, 4.0, 9.0];
let b = polyfit(x, y, 2);
assert_eq(b[2], 1.0);
```
**/
fn polyfit(x, Y, n) {
    let x0 = ones(1, x.len)[0];
    let X = [x0];
    for i in 1..=n {
        let xinc = x.map(|el| el ** i);
        print(n);
        print(xinc);
        X.push(xinc);
    }
    print(X.transpose());
    print(Y.transpose());

    regress(X.transpose(), Y.transpose()).transpose()[0]
};