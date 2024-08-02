use std::io::Cursor;

use rlox::run_file;

#[test]
fn var1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let mut writer: Box<dyn std::io::Write> = Box::new(Cursor::new(vec![]));
    run_file(
        "tests/samples/variables_1.txt",
        &mut writer,
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    assert!(!had_error);
    assert!(!had_runtime_error);
}
