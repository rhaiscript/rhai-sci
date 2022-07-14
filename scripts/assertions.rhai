fn assert(expression, error) {
    if !expression {
        throw error;
    }
}

fn assert_eq(a, b, error) {
    if a != b {
        throw error;
    }
}

fn assert_neq(a, b, error) {
    if a == b {
        throw error;
    }
}

fn assert(expression) {
    assert(expression, "Expression does not evaluate to true")
}

fn assert_eq(a, b) {
    assert_eq(a, b, "Arguments are not equal")
}

fn assert_neq(a, b) {
    assert_neq(a, b, "Arguments are equal")
}