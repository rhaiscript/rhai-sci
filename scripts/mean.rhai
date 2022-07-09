fn mean(arr) {
    arr.reduce(|sum, v| sum + v*1.0, 0)/arr.len()
};