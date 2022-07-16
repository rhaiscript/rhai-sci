// Returns the variance of a 1-D array.
fn variance(arr) {
    let m = mean(arr);
    arr.reduce(|sum, v| sum + (v - m)**2, 0)/(arr.len() - 1)
};