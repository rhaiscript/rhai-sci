/** Returns the highest value between a pair of numbers (if called with two arguments) or in a 1-D
   array (if called with a single `Array`-type argument).
**/

fn max(a, b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
};

fn max(arr) {
    arr.sort();
    arr[-1]
};