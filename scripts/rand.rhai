/* Create a matrix filled with random values between 0 and 1. Can be called with either no argument (returning a single value),
   one int argument (creating a square matrix), on array argument (indicating the dimensions, such as that which is returned by `[size]`)
   or two arguments (specifying the number of rows and columns separately).
*/

fn rand() {
    rand_float()
};

fn rand(n) {
    if type_of(n) == "array" {
        rand(n[0], n[1])
    } else {
        rand(n, n)
    }
};

fn rand(nx, ny) {
    let matrix = zeros(nx, ny);
    for i in 0..nx {
        for j in 0..ny {
            matrix[i][j] = rand();
        }
    }
    matrix
};