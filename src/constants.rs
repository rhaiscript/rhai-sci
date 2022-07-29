use rhai::plugin::*;

#[export_module]
pub mod constant_definitions {
    use crate::matrix_functions::ndims;
    use nalgebra::DMatrix;
    use polars::prelude::{CsvReader, DataType, SerReader};
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    // The ratio of a circle's circumference to its diameter.
    pub const pi: f64 = std::f64::consts::PI;

    //Speed of light in meters per second (m/s).
    pub const c: f64 = 299792458.0;

    // Euler's number.
    pub const e: f64 = std::f64::consts::E;

    // Acceleration due to gravity on Earth in meters per second per second (m/s^2).
    pub const g: f64 = 9.80665;

    // The Planck constant in Joules per Hertz (J/Hz)
    pub const h: f64 = 6.62607015e-34;

    // The golden ratio
    pub const phi: f64 = 1.61803398874989484820;

    // Newtonian gravitational constnat
    pub const G: f64 = 6.6743015e-11;
}
