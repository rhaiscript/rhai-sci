fn diag(arr) {
    try {
        let n = min(len(arr[0]), len(arr));
        let v = [];
        for i in 0..n {
            v.push(arr[i][i]);
        }
        return v;
    }
    catch {
        let n = len(arr);
        let matrix = zeros(n);
        for i in 0..n {
            matrix[i][i] = 1.0*arr[i];
        }
        return matrix;
    }
};