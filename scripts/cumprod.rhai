/// Returns an array representing the cumulative product of a 1-D array.
fn cumprod(arr) {
    let new_arr = [];
    let prod = 1;
    for el in arr {
        prod *= el;
        new_arr.push(prod);
    }
    new_arr
};