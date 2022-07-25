/**
Assert that a statement is true, and throw a custom error if it is not.
```javascript
assert(2 == 2, "Oh no!");
```
**/
fn assert(expression, error) {
    if !expression {
        throw error;
    } else {
        return true;
    }
}

/**
Assert that two arguments are equal, and throw a custom error if it is not.
```javascript
assert_eq(2, 2, "Oh no!");
```
**/
fn assert_eq(a, b, error) {
    if a != b {
        print(a);
        print(b);
        throw error;
    } else {
        return true;
    }
}

/**
Assert that two arguments are non-equal, and throw a custom error if it is not.
```javascript
assert_neq(2, 1, "Oh no!");
```
**/
fn assert_neq(a, b, error) {
    if a == b {
        print(a);
        print(b);
        throw error;
    }else {
        return true;
    }
}

/**
Assert that a statement is true and throw an error if it is not.
```javascript
assert(2==2);
```
**/
fn assert(expression) {
    assert(expression, "Expression does not evaluate to true")
}

/** Assert that two arguments are equal and throw an error if they are not.
```javascript
assert_eq(2, 2);
```
**/
fn assert_eq(a, b) {
    assert_eq(a, b, "Arguments are not equal")
}

/**
Assert that two arguments are non-equal and throw an error if they are not.
```javascript
assert_neq(2, 1);
```
**/
fn assert_neq(a, b) {
    assert_neq(a, b, "Arguments are equal")
}
