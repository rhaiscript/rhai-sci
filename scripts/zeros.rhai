fn zeros(nx, ny) {
    let row = [];
    row.pad(ny, 0.0);

    let matrix = [];
    matrix.pad(nx, row);

    matrix
};

fn zeros(n) {
    if type_of(n) == "array" {
        zeros(n[0], n[1])
    } else {
        zeros(n, n)
    }
};