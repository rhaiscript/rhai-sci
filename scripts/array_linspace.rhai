/**
Returns an array containing a number of elements linearly spaced between two bounds.
```javascript
let x = linspace(1, 2, 5);
assert_eq(x, [1.0, 1.25, 1.5, 1.75, 2.0]);
```
**/
fn linspace(x1, x2, n) {
    let arr = [1.0*x1];
    let int = (1.0*x2 - x1)/(n-1);
    for i in 0..(n-2) {
        arr.push(arr[-1] + int)
    }
    arr.push(1.0*x2);
    arr
};