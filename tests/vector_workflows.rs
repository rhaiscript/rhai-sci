use rhai::{packages::Package, Engine};
use rhai_sci::SciPackage;

fn engine() -> Engine {
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine
}

#[test]
fn statistics_and_signal_operations_accept_every_vector_orientation() {
    let engine = engine();
    for values in ["[1, 2, 3]", "[1, 2.0, 3]"] {
        for vector in [
            values.to_string(),
            format!("row({values})"),
            format!("col({values})"),
        ] {
            let script = format!(
                r#"
                let samples = {vector};
                let before = samples;
                assert_eq(mean(samples), 2.0);
                assert_eq(std(samples), 1.0);
                assert_eq(median(samples), 2.0);
                assert_approx_eq(min(samples).to_float(), 1.0);
                assert_approx_eq(max(samples).to_float(), 3.0);
                assert_eq(argmax(samples), 2);
                assert_eq(argmin(samples), 0);
                assert_approx_eq(movmean(samples, 3), [1.5, 2.0, 2.5]);
                assert_approx_eq(cumsum(samples), [1.0, 3.0, 6.0]);
                assert_approx_eq(diff(samples), [1.0, 1.0]);
                assert_eq(samples, before);
            "#
            );
            engine
                .run(&script)
                .unwrap_or_else(|error| panic!("{vector}: {error}"));
        }
    }
}

#[test]
fn paired_sample_operations_compare_vector_lengths_after_normalizing() {
    engine()
        .run(
            r#"
        let x = row([0, 1, 2]);
        let y = col([1, 3.0, 5]);
        assert_eq(trapz(x, y), 6.0);
        assert_approx_eq(cumtrapz(x, y), [0.0, 2.0, 6.0]);
        assert_eq(interp1(x, y, 0.5), 2.0);
    "#,
        )
        .unwrap();
}

#[test]
fn malformed_vectors_return_script_errors() {
    let engine = engine();
    for data in [
        "[[1], [2, 3]]",
        "[[1, 2], [3, 4]]",
        "[1, [2]]",
        "[[1, \"x\"]]",
        "[]",
        "[[]]",
    ] {
        for function in ["mean", "median", "std", "max", "argmax"] {
            let script = format!("{function}({data})");
            assert!(engine.eval::<rhai::Dynamic>(&script).is_err(), "{script}");
        }
    }
    for script in [
        "trapz(row([1, 2]), col([1, 2, 3]))",
        "cumtrapz(row([1, 2]), row([1, 2, 3]))",
        "interp1(row([1, 2]), row([1, 2, 3]), 1.5)",
    ] {
        assert!(engine.eval::<rhai::Dynamic>(script).is_err(), "{script}");
    }
}

#[test]
fn integer_statistics_preserve_values_above_float_precision() {
    engine()
        .run(
            r#"
        let n = 9007199254740993;
        assert_eq(max(col([n, n - 1])), n);
        assert_eq(argmax(col([n - 1, n])), 1);
        assert_eq(sum(row([n, 0])), n);
    "#,
        )
        .unwrap();
}

#[test]
fn empty_list_identities_remain_available() {
    engine()
        .run(
            r#"
        assert_eq(sum([]), 0);
        assert_eq(prod([]), 1);
        assert_eq(diff([]), []);
        assert_eq(unique([]), []);
        assert_eq(size([]), [0]);
        assert_eq(size([[]]), [1, 0]);
    "#,
        )
        .unwrap();
}
