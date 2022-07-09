fn maxk(arr, k) {
    arr.sort();
    let L = arr.len();
    arr.extract((L-k)..L);
};