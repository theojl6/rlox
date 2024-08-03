use std::io::Cursor;

use rlox::run_file;

#[test]
fn inheritance_1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let mut writer = Cursor::new(Vec::<u8>::new());
    run_file(
        "tests/samples/inheritance_1.txt",
        &mut writer,
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "Fry until golden brown.\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}

#[test]
fn inheritance_err_1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let mut writer = Cursor::new(Vec::<u8>::new());
    run_file(
        "tests/samples/inheritance_err_1.txt",
        &mut writer,
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "1\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}
