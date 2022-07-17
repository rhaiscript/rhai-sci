/**
Concatenates two arrays horizontally
```javascript
let arr1 = rand(3);
let arr2 = rand(3);
let combined = horzcat(arr1, arr2);
assert_eq(size(combined), [3, 6]);
```
**/
fn horzcat(arr1, arr2) {
    transpose(transpose(arr1) + transpose(arr2))
};