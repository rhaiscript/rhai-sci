fn numel(arr) {
    size(arr).reduce(|sum, v| sum*v, 1)
};