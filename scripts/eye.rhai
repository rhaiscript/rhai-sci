fn eye(n) {
    eye(n, n)
};

fn eye(nx, ny) {
    let matrix = zeros(nx, ny);
    for i in 0..min(nx, ny) {
        matrix[i][i] = 1.0;
    }
    matrix
};

