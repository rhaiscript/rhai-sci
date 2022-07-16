/// Reverse the columns in a matrix.
fn flipud(arr) {
    transpose(fliplr(transpose(arr)))
};