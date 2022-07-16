// Returns the argument of the smallest element in a 1-D array.
fn argmin(arr) {
    let original = arr;
    arr.sort();
    original.index_of(arr[0])
};