/// Returns the approximate integral of the curve defined by Y and X using the trapezoidal method.
fn trapz(x, y) {
    let trapsum = 0.0;
    for i in 1..len(x) {
        trapsum +=(y[i] - y[i-1])*(x[i]-x[i-1]);
    }
    trapsum
};

fn trapz(y) {
    let x = linspace(1, len(y), len(y));
    trapz(x, y)
};
