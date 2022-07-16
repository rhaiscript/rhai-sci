/// Returns the inter-quartile range for a 1-D array.
fn iqr(arr) {
    prctile(arr, 75) - prctile(arr, 25)
};