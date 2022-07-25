/**
Performs set intersection of two arrays
```javascript
let set1 = [7, 1, 7, 7, 4];
let set2 = [7, 0, 4, 4, 0];
let x = intersect(set1, set2);
assert_eq(x, [4, 7]);
```
**/
fn intersect(arr1, arr2) {
    unique(
        arr1.filter(
            |v| v in arr2
       )
   )
};