/// Returns the number of elements in an array.
fn numel(arr) {
    size(arr).reduce(|sum, v| sum*v, 1)
};