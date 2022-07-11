fn cummin(arr) {
    let new_arr = [];
    let current_min = arr[0];
    for el in arr {
        current_min = min(current_min, el);
        new_arr.push(current_min);
    }
    new_arr
};