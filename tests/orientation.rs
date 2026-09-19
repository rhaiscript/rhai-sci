use rhai::{Array, Dynamic, FLOAT};
use rhai_sci::matrix::RhaiMatrix;
use rhai_sci::matrix_functions;
use rhai_sci::validation_functions::{is_column_vector, is_row_vector};

#[test]
fn row_column_constructors_and_orientation() {
    let data: Array = vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ];
    let row = RhaiMatrix::row_vector(data.clone());
    let column = RhaiMatrix::column_vector(data.clone());
    let mut row_to_col = row.as_column().unwrap().to_array();
    assert!(is_column_vector(&mut row_to_col));
    let mut col_to_row = column.as_row().unwrap().to_array();
    assert!(is_row_vector(&mut col_to_row));
}

#[test]
fn validate_orientation_helpers() {
    let mut row: Array = vec![Dynamic::from_array(vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
    ])];
    let column: Array = vec![
        Dynamic::from_array(vec![Dynamic::from_int(1)]),
        Dynamic::from_array(vec![Dynamic::from_int(2)]),
    ];
    assert!(is_row_vector(&mut row.clone()));
    assert!(!is_row_vector(&mut column.clone()));
    assert!(is_column_vector(&mut column.clone()));
    assert!(!is_column_vector(&mut row));
}

#[test]
fn eye_vector_size_matches_scalar_size() {
    for size in [1, 2, 5] {
        let scalar = matrix_functions::eye_single_input(Dynamic::from_int(size))
            .expect("scalar eye should succeed");
        let vector =
            matrix_functions::eye_single_input(Dynamic::from_array(vec![Dynamic::from_int(size)]))
                .expect("vector eye should succeed");
        assert_eq!(
            normalize_matrix(&scalar),
            normalize_matrix(&vector),
            "size {size} should match",
        );
    }
}

#[test]
fn eye_vector_two_dimensions_remains_rectangular() {
    let rectangular = matrix_functions::eye_single_input(Dynamic::from_array(vec![
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ]))
    .expect("rectangular eye should succeed");
    assert_eq!(
        normalize_matrix(&rectangular),
        normalize_matrix(&matrix_functions::eye_double_input(2, 3)),
    );
}

fn normalize_matrix(matrix: &Array) -> Vec<Vec<FLOAT>> {
    matrix
        .iter()
        .map(|row| {
            row.clone()
                .into_array()
                .expect("matrix rows should be arrays")
                .into_iter()
                .map(|value| {
                    if value.is_float() {
                        value.as_float().expect("value is float")
                    } else {
                        value.as_int().expect("value is int") as FLOAT
                    }
                })
                .collect()
        })
        .collect()
}
