// Returns the root mean square of a 1-D array.
fn rms(arr) {
    sqrt(1.0*arr.reduce(|sum, v| sum + v*v, 0)/len(arr))
};