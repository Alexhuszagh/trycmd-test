const DATA: &str = r#"Rust's package manager

USAGE:
"#;

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/failed-replacement.md")
        .insert_var("[REPLACE]", DATA)
        .unwrap();
}
