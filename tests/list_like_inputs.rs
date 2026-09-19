use rhai::{Array, Dynamic, FLOAT};
use rhai_sci::moving_functions::movmean;
use rhai_sci::stats::argmax;

fn row_vector(values: &[i64]) -> Array {
    let row: Array = values
        .iter()
        .map(|value| Dynamic::from_int(*value))
        .collect();
    vec![Dynamic::from_array(row)]
}

fn column_vector(values: &[i64]) -> Array {
    values
        .iter()
        .map(|value| Dynamic::from_array(vec![Dynamic::from_int(*value)]))
        .collect()
}

fn array_to_floats(values: Array) -> Vec<FLOAT> {
    values
        .into_iter()
        .map(|value| value.as_float().unwrap())
        .collect()
}

#[test]
fn movmean_accepts_row_and_column_vectors() {
    let mut row = row_vector(&[1, 2, 3, 4]);
    let mut column = column_vector(&[1, 2, 3, 4]);

    let expected = vec![1.5, 2.0, 3.0, 3.5];

    let row_result = movmean(&mut row, 3).unwrap();
    assert_eq!(array_to_floats(row_result), expected);

    let column_result = movmean(&mut column, 3).unwrap();
    assert_eq!(array_to_floats(column_result), expected);
}

#[test]
fn argmax_accepts_row_and_column_vectors() {
    let mut row = row_vector(&[1, 9, 3]);
    let mut column = column_vector(&[1, 9, 3]);

    let row_index = argmax(&mut row).unwrap();
    let column_index = argmax(&mut column).unwrap();

    assert_eq!(row_index.as_int().unwrap(), 1);
    assert_eq!(column_index.as_int().unwrap(), 1);
}
