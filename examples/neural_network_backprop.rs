//! Trains a tiny neural network with backpropagation using rhai-sci matrices.

fn main() {
    #[cfg(feature = "nalgebra")]
    {
        use rhai::{packages::Package, Engine, Map};
        use rhai_sci::SciPackage;

        let mut engine = Engine::new();
        engine.register_global_module(SciPackage::new().as_shared_module());

        let result: Map = engine
            .eval_file("examples/neural_network_backprop.rhai".into())
            .expect("script should run");
        println!("{result:?}");
    }
}
