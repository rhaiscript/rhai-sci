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