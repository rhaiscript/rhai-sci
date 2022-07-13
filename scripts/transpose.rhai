fn transpose(arr) {
    let s = size(arr);
    let new_arr = zeros(s[1], s[0]);
    for i in 0..len(arr) {
        for j in 0..len(arr[0]) {
            new_arr[j][i] = arr[i][j]
        }
    }
    new_arr
}

fn t() {
    let arr = this;
    let s = size(arr);
    let new_arr = zeros(s[1], s[0]);
    for i in 0..len(arr) {
        for j in 0..len(arr[0]) {
            new_arr[j][i] = arr[i][j]
        }
    }
    new_arr
}
