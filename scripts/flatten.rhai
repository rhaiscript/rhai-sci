/**
Returns the contents of an multidimensional array as a 1-D array.
```javascript
let matrix = rand(3, 5);
let flat = flatten(matrix);
assert_eq(len(flat), 15);
```
**/
fn flatten(arr) {
    let flat = [];
    for el in arr {
        if type_of(el) == "array" {
            flat += flatten(el);
        } else {
            flat += el;
        }
    }
    flat
}