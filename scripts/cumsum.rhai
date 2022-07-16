/// Returns an array representing the cumulative sum of a 1-D array.
fn cumsum(arr) {
    let new_arr = [];
    let sum = 0;
    for el in arr {
        sum += el;
        new_arr.push(sum);
    }
    new_arr
};