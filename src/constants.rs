use rhai::plugin::*;

#[export_module]
pub mod constant_definitions {
    use rhai::{Dynamic, FLOAT};

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

    // Newtonian gravitational constnat
    pub const G: FLOAT = 6.6743015e-11;
}
