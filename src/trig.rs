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

    /// Convert the argument from 3D Cartesian coordinates to polar coordinates.
    /// ```typescript
    /// assert_eq(cart2pol(1.0, 1.0, 1.0), [pi/4, sqrt(2.0), 1.0])
    /// ```
    #[rhai_fn(name = "cart2pol")]
    pub fn cart2pol3d(x: FLOAT, y: FLOAT, z: FLOAT) -> Array {
        vec![
            Dynamic::from(y.atan2(x)),
            Dynamic::from(y.hypot(x)),
            Dynamic::from(z),
        ]
    }

    /// Convert the argument from 2D Cartesian coordinates to polar coordinates.
    /// ```typescript
    /// assert_eq(cart2pol(1.0, 1.0), [pi/4, sqrt(2.0)])
    /// ```
    #[rhai_fn(name = "cart2pol")]
    pub fn cart2pol2d(x: FLOAT, y: FLOAT) -> Array {
        vec![Dynamic::from(y.atan2(x)), Dynamic::from(y.hypot(x))]
    }

    /// Convert the argument from 3D polar coordinates to Cartesian coordinates.
    /// ```typescript
    /// assert_approx_eq(pol2cart(pi/4, sqrt(2.0), 1.0), [1.0, 1.0, 1.0])
    /// ```
    #[rhai_fn(name = "pol2cart")]
    pub fn pol2cart3d(theta: FLOAT, r: FLOAT, z: FLOAT) -> Array {
        vec![
            Dynamic::from(r * theta.cos()),
            Dynamic::from(r * theta.sin()),
            Dynamic::from(z),
        ]
    }

    /// Convert the argument from 2D polar coordinates to Cartesian coordinates.
    /// ```typescript
    /// assert_approx_eq(pol2cart(pi/4, sqrt(2.0)), [1.0, 1.0])
    /// ```
    #[rhai_fn(name = "pol2cart")]
    pub fn pol2cart2d(theta: FLOAT, r: FLOAT) -> Array {
        vec![
            Dynamic::from(r * theta.cos()),
            Dynamic::from(r * theta.sin()),
        ]
    }

    /// Convert the argument from 3D Cartesian coordinates to spherical coordinates.
    /// ```typescript
    /// assert_approx_eq(cart2sph(1.0, 0.0, 1.0), [0.0, pi/4, sqrt(2.0)])
    /// ```
    #[rhai_fn(name = "cart2sph")]
    pub fn cart2sph(x: FLOAT, y: FLOAT, z: FLOAT) -> Array {
        vec![
            Dynamic::from(y.atan2(x)),
            Dynamic::from(z.atan2(y.hypot(x))),
            Dynamic::from(hypot3(x, y, z)),
        ]
    }

    /// Convert the argument from spherical coordinates to 3D Cartesian coordinates.
    /// ```typescript
    /// assert_approx_eq(sph2cart(0.0, pi/4, sqrt(2.0)), [1.0, 0.0, 1.0])
    /// ```
    #[rhai_fn(name = "sph2cart")]
    pub fn sph2cart(azimuth: FLOAT, elevation: FLOAT, r: FLOAT) -> Array {
        vec![
            Dynamic::from(r * elevation.cos() * azimuth.cos()),
            Dynamic::from(r * elevation.cos() * azimuth.sin()),
            Dynamic::from(r * elevation.sin()),
        ]
    }

    /// Extends the built-in hypot function to compute distance in 3D cartesian space
    /// ```typescript
    /// assert_eq(hypot(2.0, 3.0, 6.0), 7.0);
    /// ```
    #[rhai_fn(name = "hypot")]
    pub fn hypot3(x: FLOAT, y: FLOAT, z: FLOAT) -> FLOAT {
        (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt()
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
    /// ```typescript
    /// assert_eq(sinhd(0.0), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(sinhd(10.0), sinh(10.0*pi/180.0))
    /// ```
    #[rhai_fn(name = "sinhd")]
    pub fn sinhd(degrees: FLOAT) -> FLOAT {
        FLOAT::sinh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic sine in degrees
    /// ```typescript
    /// assert_eq(asinhd(0.0), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(asinhd(10.0), 180.0/pi*asinh(10.0))
    /// ```
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
    /// ```typescript
    /// assert_eq(coshd(0.0), 1.0)
    /// ```
    /// ```typescript
    /// assert_eq(coshd(10.0), cosh(10.0*pi/180.0))
    /// ```
    #[rhai_fn(name = "coshd")]
    pub fn coshd(degrees: FLOAT) -> FLOAT {
        FLOAT::cosh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic cosine in degrees
    /// ```typescript
    /// assert_eq(acoshd(1.0), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(acoshd(10.0), 180.0/pi*acosh(10.0))
    /// ```
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
    /// ```typescript
    /// assert_eq(tanhd(0.0), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(tanhd(10.0), tanh(10.0*pi/180.0))
    /// ```
    #[rhai_fn(name = "tanhd")]
    pub fn tanhd(degrees: FLOAT) -> FLOAT {
        FLOAT::tanh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic tangent in degrees
    /// ```typescript
    /// assert_eq(atanhd(0.0), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(atanhd(10.0), 180.0/pi*atanh(10.0))
    /// ```
    #[rhai_fn(name = "atanhd")]
    pub fn atanhd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atanh(x))
    }

    /// Returns the cosecant of the argument given in radians
    /// ```typescript
    /// assert_eq(csc(-pi/2), -1.0)
    /// ```
    /// ```typescript
    /// assert_eq(csc(0.0), inf)
    /// ```
    /// ```typescript
    /// assert_eq(csc(pi/2), 1.0)
    /// ```
    #[rhai_fn(name = "csc")]
    pub fn csc(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::sin(radians)
    }

    /// Returns the cosecant of the argument given in degrees
    /// ```typescript
    /// assert_eq(cscd(-90.0), -1.0)
    /// ```
    /// ```typescript
    /// assert_eq(cscd(0.0), inf)
    /// ```
    /// ```typescript
    /// assert_eq(cscd(90.0), 1.0)
    /// ```
    #[rhai_fn(name = "cscd")]
    pub fn cscd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::sin(deg2rad(degrees))
    }

    /// Returns the inverse cosecant in radians
    /// ```typescript
    /// assert_eq(acsc(-1.0), -pi/2)
    /// ```
    /// ```typescript
    /// assert_eq(acsc(inf), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(acsc(1.0), pi/2)
    /// ```
    #[rhai_fn(name = "acsc")]
    pub fn acsc(x: FLOAT) -> FLOAT {
        FLOAT::asin(1.0 / x)
    }

    /// Returns the inverse cosecant in degrees
    /// ```typescript
    /// assert_eq(acscd(-1.0), -90.0)
    /// ```
    /// ```typescript
    /// assert_eq(acscd(inf), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(acscd(1.0), 90.0)
    /// ```
    #[rhai_fn(name = "acscd")]
    pub fn acscd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::asin(1.0 / x))
    }

    /// Returns the hyperbolic cosecant of the argument given in radians
    /// ```typescript
    /// assert_eq(csch(0.0), inf)
    /// ```
    /// ```typescript
    /// assert_eq(csch(10.0), 1.0/sinh(10.0))
    /// ```
    /// ```typescript
    /// assert_eq(csch(pi/2), 1.0/sinh(pi/2))
    /// ```
    #[rhai_fn(name = "csch")]
    pub fn csch(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::sinh(radians)
    }

    /// Returns the hyperbolic cosecant of the argument given in degrees
    /// ```typescript
    /// assert_eq(cschd(0.0), inf)
    /// ```
    /// ```typescript
    /// assert_eq(cschd(10.0), 1.0/sinhd(10.0))
    /// ```
    /// ```typescript
    /// assert_eq(cschd(90.0), 1.0/sinhd(90.0))
    /// ```
    #[rhai_fn(name = "cschd")]
    pub fn cschd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::sinh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic cosecant in radians
    /// ```typescript
    /// assert_eq(acsch(inf), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(acsch(1.0), asinh(1.0))
    /// ```
    /// ```typescript
    /// assert_eq(acsch(-1.0), asinh(-1.0))
    /// ```
    #[rhai_fn(name = "acsch")]
    pub fn acsch(x: FLOAT) -> FLOAT {
        FLOAT::asinh(1.0 / x)
    }

    /// Returns the inverse hyperbolic cosecant in degrees
    /// ```typescript
    /// assert_eq(acschd(inf), 0.0)
    /// ```
    /// ```typescript
    /// assert_eq(acschd(1.0), asinhd(1.0))
    /// ```
    /// ```typescript
    /// assert_eq(acschd(-1.0), asinhd(-1.0))
    /// ```
    #[rhai_fn(name = "acschd")]
    pub fn acschd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::asinh(1.0 / x))
    }

    /// Returns the secant of the argument given in radians
    /// ```typescript
    /// assert_eq(sec(0.0), 1.0);
    /// ```
    /// ```typescript
    /// assert_eq(sec(pi/2), 1/cos(pi/2));
    /// ```
    /// ```typescript
    /// assert_eq(sec(pi), -1.0);
    #[rhai_fn(name = "sec")]
    pub fn sec(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::cos(radians)
    }

    /// Returns the secant of the argument given in degrees
    /// ```typescript
    /// assert_eq(secd(0.0), 1.0);
    /// ```
    /// ```typescript
    /// assert_eq(secd(90.0), 1/cosd(90.0));
    /// ```
    /// ```typescript
    /// assert_eq(secd(180.0), -1.0);
    /// ```
    #[rhai_fn(name = "secd")]
    pub fn secd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::cos(deg2rad(degrees))
    }

    /// Returns the inverse secant in radians
    /// ```typescript
    /// assert_eq(asec(1.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_eq(asec(-1.0), pi);
    /// ```
    /// ```typescript
    /// assert_eq(asec(0.5), acos(2.0));
    /// ```
    #[rhai_fn(name = "asec")]
    pub fn asec(x: FLOAT) -> FLOAT {
        FLOAT::acos(1.0 / x)
    }

    /// Returns the inverse secant in degrees
    /// ```typescript
    /// assert_eq(asecd(1.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_eq(asecd(-1.0), 180.0);
    /// ```
    /// ```typescript
    /// assert_eq(asecd(0.5), acosd(2.0));
    /// ```
    #[rhai_fn(name = "asecd")]
    pub fn asecd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::acos(1.0 / x))
    }

    /// Returns the hyperbolic secant of the argument given in radians
    /// ```typescript
    /// assert_eq(sech(0.0), 1.0);
    /// ```
    /// ```typescript
    /// assert_eq(sech(10.0), 1.0/cosh(10.0));
    /// ```
    /// ```typescript
    /// assert_eq(sech(pi/2), 1.0/cosh(pi/2));
    /// ```
    #[rhai_fn(name = "sech")]
    pub fn sech(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::cosh(radians)
    }
    /// Returns the hyperbolic secant of the argument given in degrees
    /// ```typescript
    /// assert_eq(sechd(0.0), 1.0);
    /// ```
    /// ```typescript
    /// assert_eq(sechd(10.0), 1.0/coshd(10.0));
    /// ```
    /// ```typescript
    /// assert_eq(sechd(90.0), 1.0/coshd(90.0));
    /// ```
    #[rhai_fn(name = "sechd")]
    pub fn sechd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::cosh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic secant in radians
    /// ```typescript
    /// assert_eq(asech(1.0), 0.0);
    /// ```
    /// ```typescript
    /// assert_eq(asech(0.5), acosh(2.0));
    /// ```
    /// ```typescript
    /// assert_eq(asech(0.1), acosh(10.0));
    /// ```
    #[rhai_fn(name = "asech")]
    pub fn asech(x: FLOAT) -> FLOAT {
        FLOAT::acosh(1.0 / x)
    }

    /// Returns the inverse hyperbolic secant of the argument in degrees
    /// ```typescript
    /// assert_eq(asechd(1.0), 0.0);
    /// ```
    #[rhai_fn(name = "asechd")]
    pub fn asechd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::acosh(1.0 / x))
    }

    /// Returns the cotangent of the argument given in radians
    /// ```typescript
    /// assert_approx_eq(cot(pi/4), 1.0, 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cot(pi/2), 0.0, 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cot(3*pi/4), -1.0, 1e-10);
    /// ```
    #[rhai_fn(name = "cot")]
    pub fn cot(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::tan(radians)
    }

    /// Returns the cotangent of the argument given in degrees
    /// ```typescript
    /// assert_approx_eq(cotd(45.0), 1.0, 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cotd(90.0), 0.0, 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cotd(135.0), -1.0, 1e-10);
    /// ```
    #[rhai_fn(name = "cotd")]
    pub fn cotd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::tan(deg2rad(degrees))
    }

    /// Returns the inverse of the cotangent in radians
    /// ```typescript
    /// assert_eq(acot(1.0), pi/4);
    /// ```
    /// ```typescript
    /// assert_eq(acot(-1.0), -pi/4);
    /// ```
    /// ```typescript
    /// assert_eq(acot(0.0), pi/2);
    /// ```
    #[rhai_fn(name = "acot")]
    pub fn acot(x: FLOAT) -> FLOAT {
        FLOAT::atan(1.0 / x)
    }

    /// Returns the inverse of the cotangent in degrees
    /// ```typescript
    /// assert_eq(acotd(1.0), 45.0);
    /// ```
    /// ```typescript
    /// assert_eq(acotd(-1.0), -45.0);
    /// ```
    /// ```typescript
    /// assert_eq(acotd(0.0), 90.0);
    /// ```
    #[rhai_fn(name = "acotd")]
    pub fn acotd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atan(1.0 / x))
    }

    /// Returns the hyperbolic cotangent of the argument given in radians
    /// ```typescript
    /// assert_approx_eq(coth(1.0), cosh(1.0)/sinh(1.0), 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(coth(0.5), cosh(0.5)/sinh(0.5), 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(coth(0.1), cosh(0.1)/sinh(0.1), 1e-10);
    /// ```
    #[rhai_fn(name = "coth")]
    pub fn coth(radians: FLOAT) -> FLOAT {
        1.0 / FLOAT::tanh(radians)
    }

    /// Returns the hyperbolic cotangent of the argument given in degrees
    /// ```typescript
    /// assert_approx_eq(cothd(1.0), coshd(1.0)/sinhd(1.0), 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cothd(0.5), coshd(0.5)/sinhd(0.5), 1e-10);
    /// ```
    /// ```typescript
    /// assert_approx_eq(cothd(0.1), coshd(0.1)/sinhd(0.1), 1e-10);
    /// ```
    #[rhai_fn(name = "cothd")]
    pub fn cothd(degrees: FLOAT) -> FLOAT {
        1.0 / FLOAT::tanh(deg2rad(degrees))
    }

    /// Returns the inverse hyperbolic cotangent of the argument in radians
    /// ```typescript
    /// assert_eq(acoth(1.0), atanh(1.0));
    /// ```
    /// ```typescript
    /// assert_eq(acoth(-1.0), atanh(-1.0));
    /// ```
    #[rhai_fn(name = "acoth")]
    pub fn acoth(x: FLOAT) -> FLOAT {
        FLOAT::atanh(1.0 / x)
    }

    /// Returns the inverse hyperbolic cotangent of the argument in degrees
    /// ```typescript
    /// assert_eq(acothd(1.0), atanhd(1.0));
    /// ```
    /// ```typescript
    /// assert_eq(acothd(-1.0), atanhd(-1.0));
    /// ```
    #[rhai_fn(name = "acothd")]
    pub fn acothd(x: FLOAT) -> FLOAT {
        rad2deg(FLOAT::atanh(1.0 / x))
    }
}
