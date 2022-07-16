/** This function can be used in two distinct ways.
    1. If the argument is an 2-D array, `diag` returns an array containing the diagonal of the array.
    2. If the argument is a 1-D array, `diag` returns a matrix containing the argument along the
     diagonal and zeros elsewhere.
**/

fn diag(arr) {
    if ndims(arr) == 2 {
        let n = min(len(arr[0]), len(arr));
        let v = [];
        for i in 0..n {
            v.push(arr[i][i]);
        }
        return v;
    }
    else {
        let n = len(arr);
        let matrix = zeros(n);
        for i in 0..n {
            matrix[i][i] = 1.0*arr[i];
        }
        return matrix;
    }
};