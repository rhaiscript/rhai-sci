/// Rotates a matrix 90 degrees counterclockwise.
fn rot90(mat) {
    let s = size(mat);
    let new_mat = zeros(s);
    for i in 0..s[0] {
        for j in 0..s[1] {
            print(i); print(j);
            new_mat[s[1] - j -1][i] = mat[i][j]
        }
    }
    new_mat
}

/// Rotates a matrix 90 degrees counterclockwise by `k` increments.
fn rot90(mat, k) {
    for i in 0..k {
        mat = rot90(mat);
    }
    mat
}
