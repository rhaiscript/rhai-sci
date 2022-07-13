fn unique(arr) {
    let unique_elements = [];
    for element in arr {
        if !(element in unique_elements) {
            unique_elements.push(element)
        }
    }
    unique_elements
};