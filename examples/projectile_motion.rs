//! Simulates projectile motion using rhai-sci.

fn main() {
    use rhai::{packages::Package, Engine};
    use rhai_sci::SciPackage;

    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());

    let result: rhai::Map = engine
        .eval_file("examples/projectile_motion.rhai".into())
        .expect("script should run");
    println!("{result:?}");
}
