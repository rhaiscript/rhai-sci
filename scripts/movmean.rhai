fn movmean(arr, k) {
    let new_arr = [];
    let n = len(arr);
    for i in 0..n {
        if k.is_odd() {
            new_arr.push(
                mean(
                    arr.filter(|v, ii|
                        (ii <= min(i + (k-1)/2, n)) &&
                        (ii >= max(i - (k-1)/2, 0))
                    )
                )
            )
        }
    }
    new_arr
}