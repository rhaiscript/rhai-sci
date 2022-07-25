use rhai::plugin::*;

#[export_module]
pub mod assert_functions {
    use rhai::{Array, Dynamic, EvalAltResult, Position, FLOAT, INT};

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
                format!("The comparison is not true."),
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
                    "The left-hand side ({}) and right-hand side ({}) do not have the same type.",
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
                format!("The left-hand side and right-hand side are not equal."),
                Position::NONE,
            )
            .into())
        }
    }

    /// Assert that two arguments are non-equal and throw an error if they are not.
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
                    "The left-hand side ({}) and right-hand side ({}) do not have the same type.",
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
                format!("The left-hand side and right-hand side are equal."),
                Position::NONE,
            )
            .into())
        }
    }
}
