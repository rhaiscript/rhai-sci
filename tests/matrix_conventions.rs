use rhai::{packages::Package, Array, Dynamic, Engine, EvalAltResult};
use rhai_sci::SciPackage;

#[test]
fn constructors_preserve_values_and_convert_vector_orientation() {
    for script in ["row([1, 2.5, 3])", "row(col([1, 2.5, 3]))"] {
        assert_matrix_eq(eval_array(script).unwrap(), &[&[1.0, 2.5, 3.0]]);
    }
    for script in ["col([1, 2.5, 3])", "col(row([1, 2.5, 3]))"] {
        assert_matrix_eq(eval_array(script).unwrap(), &[&[1.0], &[2.5], &[3.0]]);
    }
    let values = eval_array("mat([[1, 2.5]])").unwrap()[0]
        .clone()
        .into_array()
        .unwrap();
    assert!(values[0].is_int());
    assert!(values[1].is_float());
}

#[test]
fn dot_is_a_scalar_inner_product_independent_of_orientation() {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    for left in ["[1, 2]", "row([1, 2])", "col([1, 2])"] {
        for right in ["[3, 4.0]", "row([3, 4.0])", "col([3, 4.0])"] {
            let value = engine
                .eval::<f64>(&format!("dot({left}, {right})"))
                .unwrap();
            assert_eq!(value, 11.0);
        }
    }
    assert_eq!(
        engine.eval::<f64>("col([1, 2]).dot(row([3, 4]))").unwrap(),
        11.0
    );
    for script in [
        "dot([1], [1, 2])",
        "dot([], [])",
        "dot([[1, 2], [3, 4]], [1, 2])",
        "dot([1, \"x\"], [1, 2])",
    ] {
        assert!(engine.eval::<Dynamic>(script).is_err(), "{script}");
    }
}

#[cfg(feature = "nalgebra")]
#[test]
fn matrix_products_and_concatenation_compose_with_constructors() {
    assert_matrix_eq(
        eval_array("mtimes(mat([[1, 2], [3, 4]]), col([5, 6]))").unwrap(),
        &[&[17.0], &[39.0]],
    );
    assert_matrix_eq(
        eval_array("transpose(row([1, 2]))").unwrap(),
        &[&[1.0], &[2.0]],
    );
    assert_matrix_eq(
        eval_array("transpose(col([1, 2]))").unwrap(),
        &[&[1.0, 2.0]],
    );
    assert_matrix_eq(
        eval_array("transpose(transpose(col([1, 2])))").unwrap(),
        &[&[1.0], &[2.0]],
    );
    assert_matrix_eq(
        eval_array("mtimes(row([1, 2]), col([3, 4]))").unwrap(),
        &[&[11.0]],
    );
    assert_matrix_eq(
        eval_array("horzcat(mat([[1, 2], [3, 4]]), col([5, 6]))").unwrap(),
        &[&[1.0, 2.0, 5.0], &[3.0, 4.0, 6.0]],
    );
    assert_matrix_eq(
        eval_array("vertcat(mat([[1, 2], [3, 4]]), row([5, 6]))").unwrap(),
        &[&[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0]],
    );
    assert_error_contains("horzcat(row([1, 2]), col([3, 4]))", "same number of rows");
    assert_error_contains(
        "vertcat(row([1, 2]), col([3, 4]))",
        "same number of columns",
    );
    assert_error_contains("mtimes(col([1, 2]), col([3, 4]))", "not compatible");
    assert_error_contains(
        "let A = mat([[1, 2], [3, 4]]); A[1] = [3]; mtimes(A, col([1, 2]))",
        "matrix",
    );
}

#[test]
fn constructors_reject_invalid_shapes_and_values() {
    assert_error_contains("mat([[1, 2], [3]])", "equal length");
    assert_error_contains("mat([[1, \"x\"]])", "INT or FLOAT");
    assert_error_contains("mat([[]])", "nonempty");
    assert_error_contains("mat([])", "nonempty");
    assert_error_contains("row([])", "at least one value");
    assert_error_contains("col([])", "at least one value");
    for constructor in ["row", "col"] {
        assert_error_contains(&format!("{constructor}([[1], [2, 3]])"), "vector");
        assert_error_contains(&format!("{constructor}([[1, 2], [3, 4]])"), "vector");
    }
}

#[test]
fn shape_predicates_check_every_row() {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine
        .run(
            r#"
        assert_eq(is_matrix([[1], [2, 3]]), false);
        assert_eq(is_column_vector([[1], [2, 3]]), false);
        assert_eq(is_matrix([[1, 2], [3], [4, 5, 6]]), false);
        assert_eq(is_row_vector([[[1, 2]]]), false);
        assert_eq(is_numeric_list(row([1, 2.5])), true);
        assert_eq(is_numeric_list(col([1, 2.5])), true);
    "#,
        )
        .unwrap();
}

fn eval_array(script: &str) -> Result<Array, Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.eval::<Array>(script)
}

fn assert_error_contains(script: &str, expected: &str) {
    let err = eval_array(script).unwrap_err();
    match err.as_ref() {
        EvalAltResult::ErrorArithmetic(message, _) => {
            assert!(
                message.contains(expected),
                "expected error message `{message}` to contain `{expected}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

fn assert_matrix_eq(actual: Array, expected: &[&[f64]]) {
    let actual = numeric_matrix(actual);
    let expected = expected
        .iter()
        .map(|row| row.to_vec())
        .collect::<Vec<Vec<f64>>>();
    assert_eq!(actual, expected);
}

fn numeric_matrix(matrix: Array) -> Vec<Vec<f64>> {
    matrix
        .into_iter()
        .map(|row| {
            row.into_array()
                .expect("matrix rows should be arrays")
                .into_iter()
                .map(|value| {
                    if value.is_float() {
                        value.as_float().expect("value should be FLOAT")
                    } else {
                        value.as_int().expect("value should be INT") as f64
                    }
                })
                .collect()
        })
        .collect()
}
