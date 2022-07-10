fn meshgrid(x, y) {
    let nx = len(x);
    let ny = len(y);
    let X = [];
    let Y = [];
    X.pad(ny, x);
    Y.pad(nx, y);

    #{x: X, y: transpose(Y)}
};