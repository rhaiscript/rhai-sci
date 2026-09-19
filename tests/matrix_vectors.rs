#![cfg(feature = "nalgebra")]

use rhai::{Array, Dynamic};
use rhai_sci::matrix::RhaiMatrix;
use rhai_sci::matrix_functions::{horzcat, matrix_size_by_reference, transpose, vertcat};
use rhai_sci::validation_functions::{is_column_vector, is_row_vector};

#[test]
fn constructors_create_properly_oriented_vectors() {
    // Row vector constructor produces 1xN matrix
    let row_data: Array = vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ];
    let mut row = RhaiMatrix::row_vector(row_data).to_array();
    assert!(is_row_vector(&mut row));

    // Column vector constructor produces Nx1 matrix
    let column_data: Array = vec![
        Dynamic::from_int(4),
        Dynamic::from_int(5),
        Dynamic::from_int(6),
    ];
    let mut column = RhaiMatrix::column_vector(column_data).to_array();
    assert!(is_column_vector(&mut column));
}

#[test]
fn as_column_converts_row_to_column() {
    let data: Array = vec![Dynamic::from_int(1), Dynamic::from_int(2)];
    let row = RhaiMatrix::row_vector(data);
    let mut column = row.as_column().unwrap().to_array();
    assert!(is_column_vector(&mut column));
}

#[test]
fn as_row_converts_column_to_row() {
    let data: Array = vec![Dynamic::from_int(1), Dynamic::from_int(2)];
    let column = RhaiMatrix::column_vector(data);
    let mut row = column.as_row().unwrap().to_array();
    assert!(is_row_vector(&mut row));
}

#[test]
fn transpose_flips_vector_orientation() {
    let data: Array = vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ];
    let row = RhaiMatrix::row_vector(data);
    let mut transposed = transpose(row).unwrap().to_array();
    assert!(is_column_vector(&mut transposed));
}

#[test]
fn horzcat_produces_row_vector() {
    let left: Array = vec![Dynamic::from_int(1), Dynamic::from_int(2)];
    let right: Array = vec![Dynamic::from_int(3), Dynamic::from_int(4)];
    let m1 = RhaiMatrix::row_vector(left);
    let m2 = RhaiMatrix::row_vector(right);
    let mut result = horzcat(m1, m2).unwrap().to_array();
    assert!(is_row_vector(&mut result));
    let dims = matrix_size_by_reference(&mut result);
    assert_eq!(dims[0].as_int().unwrap(), 1);
    assert_eq!(dims[1].as_int().unwrap(), 4);
}

#[test]
fn vertcat_produces_column_vector() {
    let top: Array = vec![Dynamic::from_int(1), Dynamic::from_int(2)];
    let bottom: Array = vec![Dynamic::from_int(3), Dynamic::from_int(4)];
    let m1 = RhaiMatrix::column_vector(top);
    let m2 = RhaiMatrix::column_vector(bottom);
    let mut result = vertcat(m1, m2).unwrap().to_array();
    assert!(is_column_vector(&mut result));
    let dims = matrix_size_by_reference(&mut result);
    assert_eq!(dims[0].as_int().unwrap(), 4);
    assert_eq!(dims[1].as_int().unwrap(), 1);
}
