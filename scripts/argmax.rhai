// Returns the argument of the largest element in a 1-D array.
fn argmax(arr) {
    let original = arr;
    arr.sort();
    original.index_of(arr[-1])
};