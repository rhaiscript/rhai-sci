/**
Returns the index of the largest element in a 1-D array.
```javascript
let arr = [43, 42, 500];
let idx = argmax(arr);
assert_eq(idx, 2);
```
**/
fn argmax(arr) {
    let original = arr;
    arr.sort();
    original.index_of(arr[-1])
};