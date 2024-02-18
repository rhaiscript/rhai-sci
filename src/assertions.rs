use rhai::plugin::*;

#[export_module]
pub mod assert_functions {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT};

    use crate::if_list_convert_to_vec_float_and_do;

    /// Assert that a statement is true and throw an error if it is not.
    /// ```typescript
    /// assert(2==2);
    /// ```
    #[rhai_fn(name = "assert", return_raw)]
    pub fn assert(comparison: bool) -> Result<bool, Box<EvalAltResult>> {
        if comparison {
            Ok(comparison)
        } else {
            Err(EvalAltResult::ErrorArithmetic(
                "The comparison is not true".to_string(),
                Position::NONE,
            )
            .into())
        }
    }

    /// Assert that two arguments are equal and throw an error if they are not.
    /// ```typescript
    /// assert_eq(2, 2);
    /// ```
    #[rhai_fn(name = "assert_eq", return_raw)]
    pub fn assert_eq(lhs: Dynamic, rhs: Dynamic) -> Result<bool, Box<EvalAltResult>> {
        let lhs_type = lhs.type_name();
        let rhs_type = rhs.type_name();
        if lhs_type != rhs_type {
            return Err(EvalAltResult::ErrorArithmetic(
                format!(
                    "The left-hand side ({}) and right-hand side ({}) do not have the same type",
                    lhs_type, rhs_type
                ),
                Position::NONE,
            )
            .into());
        }

        let comparison = format!("{:?}", lhs) == format!("{:?}", rhs);

        if comparison {
            Ok(comparison)
        } else {
            println!("LHS: {:?}", lhs);
            println!("RHS: {:?}", rhs);
            Err(EvalAltResult::ErrorArithmetic(
                "The left-hand side and right-hand side are not equal".to_string(),
                Position::NONE,
            )
            .into())
        }
    }

    /// Assert that two arguments are unequal and throw an error if they are not.
    /// ```typescript
    /// assert_ne(2, 1);
    /// ```
    #[rhai_fn(name = "assert_ne", return_raw)]
    pub fn assert_ne(lhs: Dynamic, rhs: Dynamic) -> Result<bool, Box<EvalAltResult>> {
        let lhs_type = lhs.type_name();
        let rhs_type = rhs.type_name();
        if lhs_type != rhs_type {
            return Err(EvalAltResult::ErrorArithmetic(
                format!(
                    "The left-hand side ({}) and right-hand side ({}) do not have the same type",
                    lhs_type, rhs_type
                ),
                Position::NONE,
            )
            .into());
        }

        let comparison = format!("{:?}", lhs) != format!("{:?}", rhs);

        if comparison {
            Ok(comparison)
        } else {
            println!("LHS: {:?}", lhs);
            println!("RHS: {:?}", rhs);
            Err(EvalAltResult::ErrorArithmetic(
                "The left-hand side and right-hand side are equal".to_string(),
                Position::NONE,
            )
            .into())
        }
    }

    /// Assert that two floats are approximately equal (within `eps`) and return an error if they
    /// are not.
    /// ```typescript
    /// assert_approx_eq(2.0, 2.000000000000000001, 1e-10);
    /// ```
    #[rhai_fn(name = "assert_approx_eq", return_raw)]
    pub fn assert_approx_eq(
        lhs: FLOAT,
        rhs: FLOAT,
        eps: FLOAT,
    ) -> Result<bool, Box<EvalAltResult>> {
        if (lhs - rhs).abs() < eps {
            Ok(true)
        } else {
            println!("LHS: {:?}", lhs);
            println!("RHS: {:?}", rhs);
            Err(EvalAltResult::ErrorArithmetic(
                "The left-hand side and right-hand side are not equal".to_string(),
                Position::NONE,
            )
            .into())
        }
    }

    /// Assert that two floats are approximately equal and return an error if they
    /// are not. Use the default tolerance of 1e-10 for the comparison.
    /// ```typescript
    /// assert_approx_eq(2.0, 2.000000000000000001);
    /// ```
    #[rhai_fn(name = "assert_approx_eq", return_raw)]
    pub fn assert_approx_eq_with_default(
        lhs: FLOAT,
        rhs: FLOAT,
    ) -> Result<bool, Box<EvalAltResult>> {
        assert_approx_eq(lhs, rhs, 1e-10)
    }

    /// Assert that two arrays are approximately equal (within `eps`) and return an error if they
    /// are not.
    /// ```typescript
    /// assert_approx_eq([2.0, 2.0], [2.0, 2.000000000000000001], 1e-10);
    /// ```
    #[rhai_fn(name = "assert_approx_eq", return_raw)]
    pub fn assert_approx_eq_list(
        lhs: Array,
        rhs: Array,
        eps: FLOAT,
    ) -> Result<bool, Box<EvalAltResult>> {
        if_list_convert_to_vec_float_and_do(&mut rhs.clone(), |rhs_as_vec_float| {
            if_list_convert_to_vec_float_and_do(&mut lhs.clone(), |lhs_as_vec_float| {
                let mut result = Ok(true);
                for i in 0..rhs_as_vec_float.len() {
                    result = result.and(assert_approx_eq(
                        lhs_as_vec_float[i],
                        rhs_as_vec_float[i],
                        eps,
                    ))
                }
                result
            })
        })
    }

    /// Assert that two arrays are approximately equal and return an error if they
    /// are not. Use the default tolerance of 1e-10 for the comparison.
    /// ```typescript
    /// assert_approx_eq([2.0, 2.0], [2.0, 2.000000000000000001]);
    /// ```
    #[rhai_fn(name = "assert_approx_eq", return_raw)]
    pub fn assert_approx_eq_list_with_default(
        lhs: Array,
        rhs: Array,
    ) -> Result<bool, Box<EvalAltResult>> {
        assert_approx_eq_list(lhs, rhs, 1e-10)
    }
}
