fn argmax(arr) {
    let original = arr;
    arr.sort();
    original.index_of(arr[-1])
};

assert(argmax([43, 42, 500]) == 2, "Test failed for argmax");