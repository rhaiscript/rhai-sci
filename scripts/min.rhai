/**
Returns the lowest value from a pair of numbers.
```javascript
let m = min(43, 42);
assert_eq(m, 42);
```
**/
fn min(a, b) {
    if (a < b) {
        return a;
    } else {
        return b;
    }
};

/**
Returns the lowest value in an array of numbers.
```javascript
let data = [41, 42, -1, 7, 2];
let m = min(data);
assert_eq(m, -1);
```
**/
fn min(arr) {
    arr.sort();
    arr[0]
};