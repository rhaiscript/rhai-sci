/// Returns the maximum absolute deviation of an arry.
fn mad(arr) {
    let m = median(arr);
    median(arr.map(|v| abs(v  - m)))
}