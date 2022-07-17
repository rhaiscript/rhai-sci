/**
Applied an operation (given as a function) to the array as a moving window
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = mov(data, 3, "mad");
assert_eq(m, [0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5]);
```
```javascript
let data = [1, 2, 4, -1, -2, -3, -1, 3, 2, 1];
let m = mov(data, 3, "max");
assert_eq(m, [2, 4, 4, 4, -1, -1, 3, 3, 3, 2]);
```
**/
fn mov(arr, k, function_name) {
    let f = Fn(function_name);
    let new_arr = [];
    let n = len(arr);
    for i in 0..n {
        new_arr.push(
            f.call(
                arr.extract(
                    if k.is_odd() {
                        max(i - (k-1)/2, 0)..=
                        min(i + (k-1)/2, n)
                    } else {
                        max(i - k/2, 0)..=
                        min(i + k/2-1, n)
                    }
                )
            )
        )
    }
    new_arr
};