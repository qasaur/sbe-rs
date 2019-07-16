static SBE_SCHEMA_PATH: Option<&'static str> = option_env!("RUST_SBE_SCHEMA_PATH");

fn main() {
    match SBE_SCHEMA_PATH {
        None => println!("skipping code generation"),
        Some(path) => initialise(path),
    }
}

fn initialise(path: &'static str) {
    initialise_cargo(path)
}

fn initialise_cargo(path: &'static str) {
    println!("cargo:rerun-if-changed={}", path)
}
