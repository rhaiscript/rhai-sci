// Sums the elements of a 1-D array. Can be called as a function or as an array method.

fn sum(arr) {
    arr.reduce(|sum, v| sum + v, 0)
}

fn sum() {
    sum(this)
}