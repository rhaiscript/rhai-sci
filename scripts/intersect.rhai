// Performs set intersection of two arrays
fn intersect(arr1, arr2) {
    unique(
        arr1.filter(
            |v| v in arr2
       )
   )
};