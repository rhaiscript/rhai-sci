/**
Returns the mode of a 1-D array.
```javascript
let data = [1, 2, 2, 2, 2, 3];
let m = mode(data);
assert_eq(m, 2);
```
**/
fn mode(arr) {
    arr.sort();
    let unique = arr;
    unique.dedup();
    let count = []; count.pad(unique.len(), 0);
    for el in arr {
        let idx = unique.index_of(el);
        count[idx] = count[idx] + 1;
    }
    let mode_idx = count.index_of(max(count));
    unique[mode_idx]
};