/**
asdf
```javascript
assert_eq(diag([1.0, 2.0]).index([1, 1]), 2.0)
```
**/
fn index(indices) {
    let matrix = this;
    index_internal(matrix, indices)
}

private fn index_internal(matrix, indices) {
    if type_of(matrix[0]) == "array" {
        return index_internal(matrix[indices[0]], indices.drain(|v, i| i > 0));
    } else {
        return matrix[indices[0]];
    }
}