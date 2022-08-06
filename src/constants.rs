use rhai::plugin::*;

#[export_module]
pub mod constant_definitions {
    use rhai::FLOAT;

    // The ratio of a circle's circumference to its diameter.
    pub const pi: FLOAT = 3.14159265358979323846264338327950288;

    //Speed of light in meters per second (m/s).
    pub const c: FLOAT = 299792458.0;

    // Euler's number.
    pub const e: FLOAT = 2.71828182845904523536028747135266250;

    // Acceleration due to gravity on Earth in meters per second per second (m/s^2).
    pub const g: FLOAT = 9.80665;

    // The Planck constant in Joules per Hertz (J/Hz)
    pub const h: FLOAT = 6.62607015e-34;

    // The golden ratio
    pub const phi: FLOAT = 1.61803398874989484820;

    // Newtonian gravitational constant
    pub const G: FLOAT = 6.6743015e-11;

    #[rhai_fn(name = "_____CONSTANTS_____")]
    /// Physical constants for
    ///  ## `pi: FLOAT`
    /// The ratio of a circle's circumference to its diameter (nondimensional).
    /// ```typescript
    /// assert_eq(pi,  3.14159265358979323846264338327950288);
    /// ```
    ///  ## `c: FLOAT`
    /// The speed of light in meters per second (m/s).
    /// ```typescript
    /// assert_eq(c, 299792458.0);
    /// ```
    /// ## `e: FLOAT`
    /// Euler's number (nondimensional).
    /// ```typescript
    /// assert_eq(e, 2.71828182845904523536028747135266250);
    /// ```
    ///  ## `g: FLOAT`
    /// The acceleration due to gravity on Earth in meters per second per second (m/s^2).
    /// ```typescript
    /// assert_eq(g, 9.80665);
    /// ```
    ///  ## `h: FLOAT`
    /// The Planck constant in Joules per Hertz (J/Hz).
    /// ```typescript
    /// assert_eq(h, 6.62607015e-34);
    /// ```
    /// ## `phi: FLOAT`
    /// The golden ratio (nondimensional).
    /// ```typescript
    /// assert_eq(phi, 1.61803398874989484820);
    /// ```
    /// ## `G: FLOAT`
    /// The Newtonian gravitational constant (nondimensional).
    /// ```typescript
    /// assert_eq(G, 6.6743015e-11);
    /// ```
    pub fn constants() {}
}
