fn zeros(nx, ny) {
    let row = [];
    row.pad(ny, 0.0);

    let matrix = [];
    matrix.pad(nx, row);

    matrix
};

fn zeros(n) {
    zeros(n, n)
};