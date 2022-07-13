fn mad(arr) {
    let m = median(arr);
    median(arr.map(|v| abs(v  - m)))
}