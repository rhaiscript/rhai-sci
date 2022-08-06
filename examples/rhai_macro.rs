macro_rules! rhai {
    () => {};
    ($($t:tt)+) => {
        {
            let engine = rhai::Engine::new();
            engine.eval(
                concat!(
                    $(
                        stringify!($t), " "
                    ),+
                )
            ).unwrap()
        }
    };
}

fn main() {
    use rhai::INT;
    let x: INT = rhai!(
        fn call_me() {
            return 3;
        }

        let result = call_me();
        result
    );
    println!("{x}");
}
