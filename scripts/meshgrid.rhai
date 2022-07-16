/* Returns an object map containing 2-D grid coordinates based on the uni-axial coordinates
   contained in arguments x and y.
*/

fn meshgrid(x, y) {
    let nx = len(x);
    let ny = len(y);
    let X = [];
    let Y = [];
    X.pad(ny, x);
    Y.pad(nx, y);

    #{x: X, y: transpose(Y)}
};