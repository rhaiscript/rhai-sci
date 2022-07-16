/** Create an matrix filled with ones. Can be called with either one int argument (creating a square
    matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
    or two arguments (specifying the number of rows and columns separately).
**/

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