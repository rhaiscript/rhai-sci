// Returns an array containing a number of elements logarithmically spaced between two bounds.
fn logspace(a, b, n) {
    let exponents = linspace(a, b, n);
    exponents.map(|e| 10**e)
};