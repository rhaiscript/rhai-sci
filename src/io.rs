use rhai::plugin::*;

#[export_module]
pub mod io_functions {
    use polars::prelude::{CsvReader, DataType, SerReader};
    use rhai::serde::to_dynamic;
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString};

    #[rhai_fn(return_raw)]
    pub fn validate_and_read(file_path: ImmutableString) -> Result<Array, Box<EvalAltResult>> {
        let file_path_as_str = file_path.as_str();

        match CsvReader::from_path(file_path_as_str) {
            Ok(csv) => {
                let x = csv
                    .infer_schema(Some(10))
                    .has_header(
                        csv_sniffer::Sniffer::new()
                            .sniff_path(file_path_as_str.clone())
                            .expect("Cannot sniff file")
                            .dialect
                            .header
                            .has_header_row,
                    )
                    .finish()
                    .expect("Cannot read file as CSV")
                    .drop_nulls(None)
                    .expect("Cannot remove null values");

                // Convert into vec of vec
                let mut final_output = vec![];
                for series in x.get_columns() {
                    let col: Vec<f64> = series
                        .cast(&DataType::Float64)
                        .expect("Cannot cast to f64")
                        .f64()
                        .unwrap()
                        .into_no_null_iter()
                        .collect();
                    final_output.push(col);
                }

                let matrix_as_array = final_output
                    .into_iter()
                    .map(|x| {
                        let mut y = vec![];
                        for el in &x {
                            y.push(*el as f64);
                        }
                        to_dynamic(&y).unwrap()
                    })
                    .collect::<Vec<Dynamic>>();

                Ok(matrix_as_array)
            }
            Err(_) => {
                if let Ok(_) = url::Url::parse(file_path_as_str) {
                    let file_contents = minreq::get(file_path_as_str)
                        .send()
                        .expect("Could not open URL");
                    let temp = temp_file::with_contents(file_contents.as_bytes());

                    let temp_file_name: ImmutableString = temp.path().to_str().unwrap().into();
                    validate_and_read(temp_file_name)
                } else {
                    panic!(
                        "The string {} is not a valid URL or file path.",
                        file_path_as_str
                    )
                }
            }
        }
    }
}
