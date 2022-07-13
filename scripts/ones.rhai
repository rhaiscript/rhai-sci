fn ones(nx, ny) {
    let row = [];
    row.pad(ny, 1.0);

    let matrix = [];
    matrix.pad(nx, row);

    matrix
};

fn ones(n) {
    if type_of(n) == "array" {
        ones(n[0], n[1])
    } else {
        ones(n, n)
    }
};
