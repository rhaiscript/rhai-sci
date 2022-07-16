/// Returns the cumulative approximate integral of the curve defined by Y and x using the trapezoidal method.
fn cumtrapz(x, y) {
    let trapsum = 0.0;
    let cumtrapsum = [0.0];
    for i in 1..len(x) {
        trapsum +=(y[i] - y[i-1])*(x[i]-x[i-1]);
        cumtrapsum.push(trapsum);
    }
    cumtrapsum
}

/// Returns the cumulative approximate integral of the curve defined by y, assuming that there is unit spacing.
fn cumtrapz(y) {
    let x = linspace(1, len(y), len(y));
    cumtrapz(x, y)
}

