// Transposes a matrix (called as a function with the matrix as input).
fn transpose(arr) {
    let s = size(arr);
    let new_arr = zeros(s[1], s[0]);
    for i in 0..len(arr) {
        for j in 0..len(arr[0]) {
            new_arr[j][i] = arr[i][j]
        }
    }
    new_arr
}

/// Transposes a matrix (called as a method of the matrix).
fn transpose() {
    let arr = this;
    let s = size(arr);
    let new_arr = zeros(s[1], s[0]);
    for i in 0..len(arr) {
        for j in 0..len(arr[0]) {
            new_arr[j][i] = arr[i][j]
        }
    }
    new_arr
}
