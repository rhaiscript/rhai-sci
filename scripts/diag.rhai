fn diag(arr) {
    if type_of(arr[0]) == "array" {
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