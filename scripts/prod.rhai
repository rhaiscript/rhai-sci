/// The product (multiplication) of all elements in a 1-D array.
fn prod(arr) {
    arr.reduce(|sum, v| sum*v, 1)
};