use rhai::plugin::*;

#[export_module]
pub mod set_functions {
    use rhai::{Array, Dynamic, EvalAltResult, ImmutableString, Position, FLOAT, INT};

    /// Returns the set union of two ararys.
    /// ```typescript
    /// let set1 = [7, 1, 7, 7, 4];
    /// let set2 = [7, 0, 4, 4, 0];
    /// let u = union(set1, set2);
    /// assert_eq(u, [0, 1, 4, 7]);
    /// ```
    #[rhai_fn(name = "union", return_raw)]
    fn union(arr1: Array, arr2: Array) -> Result<Array, Box<EvalAltResult>> {
        let mut x = arr1.clone();
        let mut y = arr2.clone();
        x.extend(y);
        crate::misc_functions::unique(arr1)
    }
}
