/**
Sums the elements of a 1-D array (called as a function with the array as an argument).
```javascript
let data = [1.1, 2.5, 3.4];
let s = sum(data);
assert_eq(s, 7.0);
```
**/
fn sum(arr) {
    arr.reduce(|sum, v| sum + v, 0)
}

/**
Sums the elements of a 1-D array (called as a method of the array).
```javascript
let s = [1.1, 2.5, 3.4].sum();
assert_eq(s, 7.0);
```
**/
fn sum() {
    sum(this)
}