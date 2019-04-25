use cc;

fn main() {
    cc::Build::new()
        .file("tests/reference_implementation.c")
        .compile("tab-hash-reference");
}
