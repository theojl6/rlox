use rlox::run_file;

#[test]
fn var1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    run_file(
        "tests/samples/var.txt",
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    assert!(!had_error);
    assert!(!had_runtime_error);
}
