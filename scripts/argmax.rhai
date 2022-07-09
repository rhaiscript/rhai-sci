fn argmax(arr) {
    let original = arr;
    arr.sort();
    original.index_of(arr[-1])
};