use rhai::plugin::*;

#[export_module]
#[allow(non_upper_case_globals)]
pub mod constant_definitions {
    use rhai::FLOAT;

    // The ratio of a circle's circumference to its diameter.
    #[allow(non_upper_case_globals)]
    pub const pi: FLOAT = 3.141_592_653_589_793_238_462_643_383_279_502_88;

    //Speed of light in meters per second (m/s).
    #[allow(non_upper_case_globals)]
    pub const c: FLOAT = 299_792_458.0;

    // Euler's number.
    #[allow(non_upper_case_globals)]
    pub const e: FLOAT = 2.718_281_828_459_045_235_360_287_471_352_662_50;

    // Acceleration due to gravity on Earth in meters per second per second (m/s^2).
    #[allow(non_upper_case_globals)]
    pub const g: FLOAT = 9.80665;

    // The Planck constant in Joules per Hertz (J/Hz)
    #[allow(non_upper_case_globals)]
    pub const h: FLOAT = 6.626_070_15e-34;

    // The golden ratio
    #[allow(non_upper_case_globals)]
    pub const phi: FLOAT = 1.618_033_988_749_894_848_20;

    // Newtonian gravitational constant
    pub const G: FLOAT = 6.674_301_5e-11;

    /// Physical constants useful for science.
    ///  ### `pi: FLOAT`
    /// The ratio of a circle's circumference to its diameter (non-dimensional).
    /// ```typescript
    /// assert_eq(pi, 3.14159265358979323846264338327950288);
    /// ```
    ///  ### `c: FLOAT`
    /// The speed of light in meters per second (m/s).
    /// ```typescript
    /// assert_eq(c, 299792458.0);
    /// ```
    /// ### `e: FLOAT`
    /// Euler's number (non-dimensional).
    /// ```typescript
    /// assert_eq(e, 2.71828182845904523536028747135266250);
    /// ```
    ///  ### `g: FLOAT`
    /// The acceleration due to gravity on Earth in meters per second per second (m/s^2).
    /// ```typescript
    /// assert_eq(g, 9.80665);
    /// ```
    ///  ### `h: FLOAT`
    /// The Planck constant in Joules per Hertz (J/Hz).
    /// ```typescript
    /// assert_eq(h, 6.62607015e-34);
    /// ```
    /// ### `phi: FLOAT`
    /// The golden ratio (non-dimensional).
    /// ```typescript
    /// assert_eq(phi, 1.61803398874989484820);
    /// ```
    /// ### `G: FLOAT`
    /// The Newtonian gravitational constant (non-dimensional).
    /// ```typescript
    /// assert_eq(G, 6.6743015e-11);
    /// ```
    #[rhai_fn(name = "$CONSTANTS$")]
    pub fn constants() {}
}
