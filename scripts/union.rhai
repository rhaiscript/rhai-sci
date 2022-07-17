/**
Returns the set union of two ararys.
```javascript
let set1 = [7, 1, 7, 7, 4];
let set2 = [7, 0, 4, 4, 0];
let u = union(set1, set2);
assert_eq(u, [0, 1, 4, 7]);
```
**/
fn union(arr1, arr2) {
    unique(arr1 + arr2)
};