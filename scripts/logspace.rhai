fn logspace(a, b, n) {
    let exponents = linspace(a, b, n);
    exponents.map(|e| 10**e)
};