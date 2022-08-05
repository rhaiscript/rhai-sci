use rhai::INT;

macro_rules! rhai {
    () => {};
    ($($t:tt)+) => {
        rhai_sci::eval(
            concat!(
                $(
                    stringify!($t), " "
                ),+
            )
        ).unwrap()
    };
}

fn main() {
    let x: INT = rhai!(
        let data = [43, 42, -500];
        let x = argmin(data);
        let y = argmax(data);
        y - x
    );
    println!("{x}");
}
