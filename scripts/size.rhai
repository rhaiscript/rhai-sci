/// Returns the size along each dimension of an array.
fn size(arr) {
    let s = size_me_up([], arr);
    if s[-1] == 0 {
        let x = [];
        x.pad(len(s), 0);
        return x;
    } else {
        return s;
    }
}

private fn size_me_up(size_so_far, arr) {
    if type_of(arr) == "array" {
        size_so_far.push(len(arr));
        if len(arr) == 0 {
            return size_so_far;
        } else {
            size_me_up(size_so_far, arr[0]);
        }
    } else {
        return size_so_far;
    }
};