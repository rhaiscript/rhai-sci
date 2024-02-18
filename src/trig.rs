use rhai::plugin::*;

#[export_module]
pub mod trig_functions {
    use crate::if_int_convert_to_float_and_do;
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
    pub fn sind(degrees: FLOAT) -> FLOAT {
        FLOAT::sin(deg2rad(degrees))
    }

    /// Returns the inverse sine in degrees
    /// ```typescript
    /// assert_eq(asind(-1.0), -90.0);
    /// ```
    /// ```typescript
    /// assert_eq(asind(0.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_eq(asind(1.0), 90.0);
    /// ```
    #[rhai_fn(name = "asind")]
    pub fn asind(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::asin(x))
    }

    /// Returns the hyperbolic sine of the argument given in degrees
    #[rhai_fn(name = "sinhd")]
    pub fn sinhd(degrees: FLOAT) -> FLOAT {
        FLOAT::sinh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic sine in degrees
    #[rhai_fn(name = "asinhd")]
    pub fn asinhd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::asinh(x))
    }

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
    pub fn cosd(degrees: FLOAT) -> FLOAT {
        FLOAT::cos(deg2rad(degrees))
    }

    /// Returns the inverse cosine in degrees
    /// ```typescript
    /// assert_eq(acosd(-1.0), 180.0);
    /// ```
    /// ```typescript
    /// assert_eq(acosd(0.0), 90.0);
    /// ```
    /// ```typescript
    /// assert_eq(acosd(1.0), 0.0);
    /// ```
    #[rhai_fn(name = "acosd")]
    pub fn acosd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::acos(x))
    }

    /// Returns the hyperbolic cosine of the argument given in degrees
    #[rhai_fn(name = "coshd")]
    pub fn coshd(degrees: FLOAT) -> FLOAT {
        FLOAT::cosh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic cosine in degrees
    #[rhai_fn(name = "acoshd")]
    pub fn acoshd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::acosh(x))
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
    pub fn tand(degrees: FLOAT) -> FLOAT {
        FLOAT::tan(deg2rad(degrees))
    }

    /// Returns the inverse tangent in degrees
    /// ```typescript
    /// assert_approx_eq(atand(-1.0), -45.0);
    /// ```
    /// ```typescript
    /// assert_eq(atand(0.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_approx_eq(atand(1.0), 45.0);
    /// ```
    #[rhai_fn(name = "atand")]
    pub fn atand(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atan(x))
    }

    /// Returns the inverse tangent in degrees , taking two arguments as input.
    /// ```typescript
    /// assert_approx_eq(atand(-1.0, 1.0), -45.0);
    /// ```
    /// ```typescript
    /// assert_eq(atand(0.0, 1.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_approx_eq(atand(1.0, 1.0), 45.0);
    /// ```
    #[rhai_fn(name = "atand")]
    pub fn atand2(x: FLOAT, y: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atan2(x, y))
    }

    /// Returns the hyperbolic tangent of the argument given in degrees
    #[rhai_fn(name = "tanhd")]
    pub fn tanhd(degrees: FLOAT) -> FLOAT {
        FLOAT::tanh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic tangent in degrees
    #[rhai_fn(name = "atanhd")]
    pub fn atanhd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atanh(x))
    }

    /// Returns the cosecant of the argument given in radians
    #[rhai_fn(name = "csc")]
    pub fn csc(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::sin(radians)
    }

    /// Returns the cosecant of the argument given in degrees
    #[rhai_fn(name = "cscd")]
    pub fn cscd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::sin(deg2rad(degrees))
    }

    /// Returns the inverse cosecant in radians
    #[rhai_fn(name = "acsc")]
    pub fn acsc(x: FLOAT) -> FLOAT {
        FLOAT::asin(1.0 / x)
    }

    /// Returns the inverse cosecant in degrees
    #[rhai_fn(name = "acscd")]
    pub fn acscd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::asin(1.0 / x))
    }

    /// Returns the hyperbolic cosecant of the argument given in radians
    #[rhai_fn(name = "csch")]
    pub fn csch(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::sinh(radians)
    }

    /// Returns the hyperbolic cosecant of the argument given in degrees
    #[rhai_fn(name = "cschd")]
    pub fn cschd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::sinh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic cosecant in radians
    #[rhai_fn(name = "acsch")]
    pub fn acsch(x: FLOAT) -> FLOAT {
        FLOAT::asinh(1.0 / x)
    }

    /// Returns the inverse hyperbolic cosecant in degrees
    #[rhai_fn(name = "acschd")]
    pub fn acschd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::asinh(1.0 / x))
    }

    /// Returns the secant of the argument given in radians
    #[rhai_fn(name = "sec")]
    pub fn sec(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::cos(radians)
    }

    /// Returns the secant of the argument given in degrees
    #[rhai_fn(name = "secd")]
    pub fn secd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::cos(deg2rad(degrees))
    }

    /// Returns the inverse secant in radians
    #[rhai_fn(name = "asec")]
    pub fn asec(x: FLOAT) -> FLOAT {
        FLOAT::acos(1.0 / x)
    }

    /// Returns the inverse secant in degrees
    #[rhai_fn(name = "asecd")]
    pub fn asecd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::acos(1.0 / x))
    }

    /// Returns the hyperbolic secant of the argument given in radians
    #[rhai_fn(name = "sech")]
    pub fn sech(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::cosh(radians)
    }

    /// Returns the hyperbolic secant of the argument given in degrees
    #[rhai_fn(name = "sechd")]
    pub fn sechd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::cosh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic secant in radians
    #[rhai_fn(name = "asech")]
    pub fn asech(x: FLOAT) -> FLOAT {
        FLOAT::acosh(1.0 / x)
    }

    /// Returns the inverse hyperbolic secant of the argument in degrees
    #[rhai_fn(name = "asechd")]
    pub fn asechd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::acosh(1.0 / x))
    }

    /// Returns the cotangent of the argument given in radians
    #[rhai_fn(name = "cot")]
    pub fn cot(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::tan(radians)
    }

    /// Returns the cotangent of the argument given in degrees
    #[rhai_fn(name = "cotd")]
    pub fn cotd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::tan(deg2rad(degrees))
    }

    /// Returns the inverse of the cotangent in radians
    #[rhai_fn(name = "acot")]
    pub fn acot(x: FLOAT) -> FLOAT {
        FLOAT::atan(1.0 / x)
    }

    /// Returns the inverse of the cotangent in degrees
    #[rhai_fn(name = "acotd")]
    pub fn acotd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atan(1.0 / x))
    }

    /// Returns the hyperbolic cotangent of the argument given in radians
    #[rhai_fn(name = "coth")]
    pub fn coth(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::tanh(radians)
    }

    /// Returns the hyperbolic cotangent of the argument given in degrees
    #[rhai_fn(name = "cothd")]
    pub fn cothd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::tanh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic cotangent of the argument in radians
    #[rhai_fn(name = "acoth")]
    pub fn acoth(x: FLOAT) -> FLOAT {
        FLOAT::atanh(1.0 / x)
    }

    /// Returns the inverse hyperbolic cotangent of the argument in degrees
    #[rhai_fn(name = "acothd")]
    pub fn acothd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atanh(1.0 / x))
    }
}
