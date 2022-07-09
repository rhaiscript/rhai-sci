fn ones(nx, ny) {
    let row = [];
    row.pad(ny, 1.0);

    let matrix = [];
    matrix.pad(nx, row);

    matrix
};

fn ones(n) {
    ones(n, n)
};