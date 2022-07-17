/**
Concatenates two arrays horizontally
```javascript
let arr1 = rand(3);
let arr2 = rand(3);
let combined = vertcat(arr1, arr2);
assert_eq(size(combined), [6, 3]);
```
**/
fn vertcat(arr1, arr2) {
    arr1 + arr2
};