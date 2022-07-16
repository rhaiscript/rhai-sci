// Returns the k highest values from a 1-D array.
fn maxk(arr, k) {
    arr.sort();
    let L = arr.len();
    arr.extract((L-k)..L);
};