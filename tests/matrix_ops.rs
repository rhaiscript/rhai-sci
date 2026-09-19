#![cfg(feature = "nalgebra")]

use rhai::{Array, Dynamic, EvalAltResult, FLOAT, INT};
use rhai_sci::matrix::RhaiMatrix;
use rhai_sci::matrix_functions::{
    horzcat, matrix_size_by_reference, meshgrid, repmat, transpose, vertcat,
};
use rhai_sci::validation_functions::{is_column_vector, is_row_vector};

#[test]
fn transpose_orients_row_vector() {
    let data: Array = vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ];
    let row = RhaiMatrix::row_vector(data);
    let mut result = transpose(row).unwrap().to_array();
    assert!(is_column_vector(&mut result));
}

#[test]
fn transpose_orients_column_vector() {
    let data: Array = vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ];
    let column = RhaiMatrix::column_vector(data);
    let mut result = transpose(column).unwrap().to_array();
    assert!(is_row_vector(&mut result));

    let row = result[0].clone().into_array().unwrap();
    let values: Vec<FLOAT> = row.into_iter().map(|d| d.as_float().unwrap()).collect();
    assert_eq!(values, vec![1.0, 2.0, 3.0]);
}

#[test]
fn horzcat_concatenates_rows() {
    let a: Array = vec![Dynamic::from_int(1), Dynamic::from_int(2)];
    let b: Array = vec![Dynamic::from_int(3), Dynamic::from_int(4)];
    let m1 = RhaiMatrix::row_vector(a);
    let m2 = RhaiMatrix::row_vector(b);
    let mut result = horzcat(m1, m2).unwrap().to_array();
    assert!(is_row_vector(&mut result));
    let row = result[0].clone().into_array().unwrap();
    let values: Vec<FLOAT> = row.into_iter().map(|d| d.as_float().unwrap()).collect();
    assert_eq!(values, vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn horzcat_column_vectors_result_in_matrix_with_two_columns() {
    let a = RhaiMatrix::column_vector(vec![Dynamic::from_int(1), Dynamic::from_int(2)]);
    let b = RhaiMatrix::column_vector(vec![Dynamic::from_int(3), Dynamic::from_int(4)]);
    let mut result = horzcat(a, b).unwrap().to_array();
    let shape = matrix_size_by_reference(&mut result);
    let dims: Vec<INT> = shape.into_iter().map(|d| d.as_int().unwrap()).collect();
    assert_eq!(dims, vec![2, 2]);
}

#[test]
fn horzcat_mixed_shapes_error_out() {
    let row = RhaiMatrix::row_vector(vec![Dynamic::from_int(1), Dynamic::from_int(2)]);
    let column = RhaiMatrix::column_vector(vec![Dynamic::from_int(3), Dynamic::from_int(4)]);
    let err = horzcat(row, column).unwrap_err();
    match err.as_ref() {
        EvalAltResult::ErrorArithmetic(message, _) => {
            assert!(message.contains("same number of rows"));
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn vertcat_concatenates_columns() {
    let a: Array = vec![
        Dynamic::from_array(vec![Dynamic::from_int(1)]),
        Dynamic::from_array(vec![Dynamic::from_int(2)]),
    ];
    let b: Array = vec![
        Dynamic::from_array(vec![Dynamic::from_int(3)]),
        Dynamic::from_array(vec![Dynamic::from_int(4)]),
    ];
    let m1 = RhaiMatrix::from_array(a);
    let m2 = RhaiMatrix::from_array(b);
    let mut result = vertcat(m1, m2).unwrap().to_array();
    assert!(is_column_vector(&mut result));
}

#[test]
fn vertcat_row_vectors_result_in_matrix_with_two_rows() {
    let m1 = RhaiMatrix::row_vector(vec![Dynamic::from_int(1), Dynamic::from_int(2)]);
    let m2 = RhaiMatrix::row_vector(vec![Dynamic::from_int(3), Dynamic::from_int(4)]);
    let mut result = vertcat(m1, m2).unwrap().to_array();
    let shape = matrix_size_by_reference(&mut result);
    let dims: Vec<INT> = shape.into_iter().map(|d| d.as_int().unwrap()).collect();
    assert_eq!(dims, vec![2, 2]);
}

#[test]
fn vertcat_mixed_shapes_error_out() {
    let column = RhaiMatrix::column_vector(vec![Dynamic::from_int(1), Dynamic::from_int(2)]);
    let row = RhaiMatrix::row_vector(vec![Dynamic::from_int(3), Dynamic::from_int(4)]);
    let err = vertcat(column, row).unwrap_err();
    match err.as_ref() {
        EvalAltResult::ErrorArithmetic(message, _) => {
            assert!(message.contains("same number of columns"));
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn repmat_replicates_matrix() {
    let data: Array = vec![
        Dynamic::from_array(vec![Dynamic::from_int(1), Dynamic::from_int(2)]),
        Dynamic::from_array(vec![Dynamic::from_int(3), Dynamic::from_int(4)]),
    ];
    let m = RhaiMatrix::from_array(data);
    let mut result = repmat(m, 2, 2).unwrap().to_array();
    let shape = matrix_size_by_reference(&mut result);
    let dims: Vec<INT> = shape.into_iter().map(|d| d.as_int().unwrap()).collect();
    assert_eq!(dims, vec![4, 4]);
}

#[test]
fn meshgrid_matches_matlab_shape_for_mismatched_lengths() {
    let x: Array = vec![
        Dynamic::from_int(1),
        Dynamic::from_int(2),
        Dynamic::from_int(3),
    ];
    let y: Array = vec![Dynamic::from_int(4), Dynamic::from_int(5)];
    let grid = meshgrid(x.clone(), y.clone()).unwrap();

    let x_grid = grid.get("x").unwrap().clone().into_array().unwrap();
    let y_grid = grid.get("y").unwrap().clone().into_array().unwrap();

    let mut x_grid_for_size = x_grid.clone();
    let x_shape = matrix_size_by_reference(&mut x_grid_for_size);
    let x_dims: Vec<INT> = x_shape.into_iter().map(|d| d.as_int().unwrap()).collect();
    assert_eq!(x_dims, vec![2, 3]);

    let mut y_grid_for_size = y_grid.clone();
    let y_shape = matrix_size_by_reference(&mut y_grid_for_size);
    let y_dims: Vec<INT> = y_shape.into_iter().map(|d| d.as_int().unwrap()).collect();
    assert_eq!(y_dims, vec![2, 3]);

    for row in x_grid.into_iter() {
        let row_values: Vec<INT> = row
            .into_array()
            .unwrap()
            .into_iter()
            .map(|d| d.as_int().unwrap())
            .collect();
        assert_eq!(row_values, vec![1, 2, 3]);
    }

    let y_rows: Vec<Vec<INT>> = y_grid
        .into_iter()
        .map(|row| {
            row.into_array()
                .unwrap()
                .into_iter()
                .map(|d| d.as_int().unwrap())
                .collect()
        })
        .collect();
    assert_eq!(y_rows, vec![vec![4, 4, 4], vec![5, 5, 5]]);
}

#[test]
fn meshgrid_accepts_row_vector_input() {
    let row: Array = vec![Dynamic::from_array(vec![
        Dynamic::from_int(0),
        Dynamic::from_int(1),
        Dynamic::from_int(2),
    ])];
    let y: Array = vec![Dynamic::from_int(3), Dynamic::from_int(4)];

    let grid = meshgrid(row, y).unwrap();

    let x_grid = grid.get("x").unwrap().clone().into_array().unwrap();
    let y_grid = grid.get("y").unwrap().clone().into_array().unwrap();

    for row in x_grid.into_iter() {
        let values: Vec<INT> = row
            .into_array()
            .unwrap()
            .into_iter()
            .map(|d| d.as_int().unwrap())
            .collect();
        assert_eq!(values, vec![0, 1, 2]);
    }

    let y_rows: Vec<Vec<INT>> = y_grid
        .into_iter()
        .map(|row| {
            row.into_array()
                .unwrap()
                .into_iter()
                .map(|d| d.as_int().unwrap())
                .collect()
        })
        .collect();
    assert_eq!(y_rows, vec![vec![3, 3, 3], vec![4, 4, 4]]);
}

#[test]
fn meshgrid_accepts_column_vector_inputs() {
    let column_x: Array = vec![
        Dynamic::from_array(vec![Dynamic::from_int(0)]),
        Dynamic::from_array(vec![Dynamic::from_int(1)]),
        Dynamic::from_array(vec![Dynamic::from_int(2)]),
    ];
    let column_y: Array = vec![
        Dynamic::from_array(vec![Dynamic::from_int(3)]),
        Dynamic::from_array(vec![Dynamic::from_int(4)]),
    ];

    let grid = meshgrid(column_x, column_y).unwrap();

    let x_grid = grid.get("x").unwrap().clone().into_array().unwrap();
    let y_grid = grid.get("y").unwrap().clone().into_array().unwrap();

    for row in x_grid.into_iter() {
        let values: Vec<INT> = row
            .into_array()
            .unwrap()
            .into_iter()
            .map(|d| d.as_int().unwrap())
            .collect();
        assert_eq!(values, vec![0, 1, 2]);
    }

    let y_rows: Vec<Vec<INT>> = y_grid
        .into_iter()
        .map(|row| {
            row.into_array()
                .unwrap()
                .into_iter()
                .map(|d| d.as_int().unwrap())
                .collect()
        })
        .collect();
    assert_eq!(y_rows, vec![vec![3, 3, 3], vec![4, 4, 4]]);
}
