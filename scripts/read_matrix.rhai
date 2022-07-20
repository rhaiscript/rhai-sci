/**
Reads a .csv into a matrix
```javascript
let url = "https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv";
let x = read_matrix(url);
assert_eq(size(x), [768, 9]);
```
**/
fn read_matrix(path_or_url) {
    transpose(validate_and_read(path_or_url))
};