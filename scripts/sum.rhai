/// Sums the elements of a 1-D array (called as a function with the array as an argument).
fn sum(arr) {
    arr.reduce(|sum, v| sum + v, 0)
}

/// Sums the elements of a 1-D array (called as a method of the array).
fn sum() {
    sum(this)
}