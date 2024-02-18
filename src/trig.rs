use rhai::plugin::*;

#[export_module]
pub mod trig_functions {
    use crate::{
        if_int_convert_to_float_and_do
    };
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};



    /// Converts the argument from degrees to radians
    /// ```typescript
    /// assert_eq(deg2rad(180.0), pi);
    /// ```
    #[rhai_fn(name = "deg2rad")]
    pub fn deg2rad(degrees: FLOAT) -> FLOAT {
        degrees * std::f64::consts::PI / 180.0
    }


    /// Converts the argument from radians to degrees
    /// ```typescript
    /// assert_eq(rad2deg(pi), 180.0);
    /// ```
    #[rhai_fn(name = "rad2deg")]
    pub fn rad2deg(radians: FLOAT) -> FLOAT {
        radians * 180.0 / std::f64::consts::PI
    }

    /// Returns the sine of an argument given in degrees
    /// ```typescript
    /// assert_eq(sind(0.0),  0.0);
    /// ```
    /// ```typescript
    /// assert_eq(sind(90.0), 1.0);
    /// ```
    /// ```typescript
    /// assert_approx_eq(sind(180.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_eq(sind(270.0), -1.0);
    /// ```
    #[rhai_fn(name = "sind")]
    pub fn sind(x: FLOAT) -> FLOAT {
        FLOAT::sin(deg2rad(x))
    }

    //
    /// Returns the cosine of an argument given in degrees
    /// ```typescript
    /// assert_eq(cosd(0.0), 1.0);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cosd(90.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_eq(cosd(180.0), -1.0);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cosd(270.0), 0.0);
    /// ```
    #[rhai_fn(name = "cosd")]
    pub fn cosd(x: FLOAT) -> FLOAT {
        FLOAT::cos(deg2rad(x))
    }


    /// Returns the tangent of an argument given in degrees
    /// ```typescript
    /// assert_approx_eq(tand(-45.0), -1.0);
    /// ```
    /// ```typescript
    /// assert_eq(tand(0.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_approx_eq(tand(45.0), 1.0);
    /// ```
    #[rhai_fn(name = "tand")]
    pub fn tand(x: FLOAT) -> FLOAT {
        FLOAT::tan(deg2rad(x))
    }

}
