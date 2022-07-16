/** Rotates a matrix 90 degrees counterclockwise, either in a single increments
   (if a single argument is provided), or in `k` increments (if a second argument is provided).
**/

fn rot90(arr) {
    let s = size(arr);
    let new_arr = zeros(s);
    for i in 0..s[0] {
        for j in 0..s[1] {
            print(i); print(j);
            new_arr[s[1] - j -1][i] = arr[i][j]
        }
    }
    new_arr
}

fn rot90(arr, k) {
    for i in 0..k {
        arr = rot90(arr);
    }
    arr
}
