/// Applied an operation (given as a function) to the array as a moving window
fn mov(arr, k, function_name) {
    let f = Fn(function_name);
    let new_arr = [];
    let n = len(arr);
    for i in 0..n {
        new_arr.push(
            f.call(
                arr.extract(
                    if k.is_odd() {
                        max(i - (k-1)/2, 0)..=
                        min(i + (k-1)/2, n)
                    } else {
                        max(i - k/2, 0)..=
                        min(i + k/2-1, n)
                    }
                )
            )
        )
    }
    new_arr
};