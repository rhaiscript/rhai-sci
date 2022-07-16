use std::io::Write;

fn main() {
    println!("HELLOOOOOOOO!");
    println!("cargo:rerun-if-changed=scripts");

    let paths = std::fs::read_dir("scripts").unwrap();

    let mut file = std::fs::File::create("scripts/rhai-lab-compiled.txt")
        .unwrap();

    for path in paths {
        let name = path.unwrap().path();
        if name.clone().to_str().unwrap().ends_with(".rhai") {
            let contents = std::fs::read_to_string(name).unwrap();
            write!(file, "{contents}\n\n").expect("TODO: panic message");
        }
    }
}