#[test]
fn pass_simple() {
    macrotest::expand("tests/expand/*.rs");
}
