/// Returns an array containing a number of elements linearly spaced between two bounds.
fn linspace(x1, x2, n) {
    let arr = [1.0*x1];
    let int = (1.0*x2 - x1)/(n-1);
    for i in 0..(n-2) {
        arr.push(arr[-1] + int)
    }
    arr.push(1.0*x2);
    arr
};