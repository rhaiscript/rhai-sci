/**
The product (multiplication) of all elements in a 1-D array.
```javascript
let data = [1, 2, 3, 4, 10];
let p = prod(data);
assert_eq(p, 240);
```
**/
fn prod(arr) {
    arr.reduce(|sum, v| sum*v, 1)
};