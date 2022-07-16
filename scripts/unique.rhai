// Returns an array of the unique elements in an array.
fn unique(arr) {
    arr.sort();
    arr.dedup();
    arr
};