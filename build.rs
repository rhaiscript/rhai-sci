use std::io::Write;

fn main() {
    // Update if needed
    println!("cargo:rerun-if-changed=scripts");

    // Read directory of paths
    let paths = std::fs::read_dir("scripts").unwrap();

    // Open file to write to
    let mut func_file = std::fs::File::create("scripts/rhai-lab-compiled.txt").unwrap();
    let mut test_file = std::fs::File::create("tests/rhai-lab-test.txt").unwrap();

    for path in paths {
        let name = path.unwrap().path();
        if name.clone().to_str().unwrap().ends_with(".rhai") {
            let contents = std::fs::read_to_string(name.clone()).unwrap();
            write!(func_file, "{contents}\n\n").expect("Cannot write to {func_file}");
        }
        if name.clone().to_str().unwrap().ends_with(".rhai-test") {
            let contents = std::fs::read_to_string(name.clone()).unwrap();
            write!(test_file, "{contents}\n\n").expect("Cannot write to {test_file}");
        }
    }
}