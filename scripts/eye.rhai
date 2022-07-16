/* Create an identity matrix with ones along the diagonal and zeros elsewhere. Can be called with
   either one argument (creating a square matrix) or two arguments (specifying the number of rows
   and columns separately).
*/

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

