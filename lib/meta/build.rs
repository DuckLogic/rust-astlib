use parol::build::Builder;

fn main() {
    Builder::with_cargo_script_output()
        .grammar_file("src/asdl.par")
        .generate_parser()
        .unwrap();

}