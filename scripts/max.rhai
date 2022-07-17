/**
Returns the highest value from a pair of numbers.
```javascript
let m = max(41, 42);
assert_eq(m, 42);
```
**/
fn max(a, b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
};

/**
Returns the highest value in an array of numbers.
```javascript
let data = [41, 42, -1, 7, 2];
let m = max(data);
assert_eq(m, 42);
```
**/
fn max(arr) {
    arr.sort();
    arr[-1]
};