/**
Performs ordinary least squares regression.
```javascript
let a = eye(3);
let b = ones(3);
let c = mtimes(a, b);
assert_eq(b, c);
```
**/
 private fn mtimes(a, b) {
     let b2 = transpose(b);

     let c = zeros(a.len, b[0].len);

     for i in 0..c.len {
         for j in 0..c[i].len {
             c[i][j] = 0.0;

             for z in 0..a[i].len {
                 c[i][j] += a[i][z] * b2[j][z];
             }
         }
     }

     c
 }