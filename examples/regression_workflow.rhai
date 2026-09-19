// The host loads a numeric CSV into `observations` (one observation per row).
// Columns are temperature, load, and measured response; see regression_workflow.rs.
let columns = transpose(mat(observations));
let predictors = horzcat(col(columns[0]), col(columns[1]));
let response = col(columns[2]);

// regress fits an intercept automatically; do not add a column of ones.
let fit = regress(predictors, response);
let linear_part = mtimes(predictors, col(fit.parameters));
let predictions = [];
let residual_values = [];
for i in 0..observations.len() {
    let prediction = fit.intercept + linear_part[i][0];
    predictions.push(prediction);
    residual_values.push(response[i][0] - prediction);
}

// Statistics consume the same column-vector representation as matrix operations.
let residuals = col(residual_values);
let result = #{
    "intercept": fit.intercept,
    "parameters": fit.parameters,
    "predictions": predictions,
    "residual_mean": mean(residuals),
    "residual_std": std(residuals),
    "rmse": sqrt(dot(residuals, residuals) / residual_values.len().to_float())
};
print(result);
result
