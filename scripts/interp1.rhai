fn interp1(x, y, xq) {
    x.sort();
    let b = 0;
    for idx in 0..x.len() {
        if x[idx] > xq {
            b = idx;
            break;
        }
    }
    let a = b - 1;
    y[a] + (xq - x[a])*(y[b] - y[a])/(x[b] - x[a])
};