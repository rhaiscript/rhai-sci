use rhai::plugin::*;

#[export_module]
pub mod set_functions {
    use rhai::{Array, Dynamic, EvalAltResult};

    /// Returns the set union of two arrays.
    /// ```typescript
    /// let set1 = [7, 1, 7, 7, 4];
    /// let set2 = [7, 0, 4, 4, 0];
    /// let u = union(set1, set2);
    /// assert_eq(u, [0, 1, 4, 7]);
    /// ```
    #[rhai_fn(name = "union", return_raw)]
    pub fn union(arr1: Array, arr2: Array) -> Result<Array, Box<EvalAltResult>> {
        let mut x = arr1.clone();
        let y = arr2.clone();
        x.extend(y);
        crate::misc_functions::unique(&mut x)
    }

    /// Performs set intersection of two arrays
    /// ```typescript
    ///  let set1 = [7, 1, 7, 7, 4];
    ///  let set2 = [7, 0, 4, 4, 0];
    /// let x = intersect(set1, set2);
    /// assert_eq(x, [4, 7]);
    /// ```
    #[rhai_fn(name = "intersect", return_raw)]
    pub fn intersect(arr1: Array, arr2: Array) -> Result<Array, Box<EvalAltResult>> {
        let array2 = arr2
            .into_iter()
            .map(|x| format!("{:?}", x).to_string())
            .collect::<Vec<String>>();
        let mut new_arr = vec![];
        for el in arr1 {
            if array2.contains(&format!("{:?}", el).to_string()) {
                new_arr.push(el);
            }
        }
        Ok(crate::misc_functions::unique(&mut new_arr).unwrap())
    }
}
