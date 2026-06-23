use cc;

fn main() {
    println!("cargo:rerun-if-changed=tests/reference_implementation.c");
    cc::Build::new()
        .file("tests/reference_implementation.c")
        .compile("tab-hash-reference");
}
