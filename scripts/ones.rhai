/** Create an matrix filled with ones. Can be called with either one int argument (creating a square
   matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
   or two arguments (specifying the number of rows and columns separately).
**/

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
