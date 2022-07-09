fn prctile(arr, p) {
    arr.sort();
    let x = linspace(0, 100, arr.len());
    interp1(x, arr, p)
};