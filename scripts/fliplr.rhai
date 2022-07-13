fn fliplr(arr) {
    for (row, idx) in arr {
        row.reverse();
        arr[idx] = row;
    }
    arr
};