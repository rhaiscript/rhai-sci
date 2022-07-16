// Returns the k smallest values in a 1-D array.
fn mink(arr, k) {
    arr.sort();
    arr.extract(0..k);
};