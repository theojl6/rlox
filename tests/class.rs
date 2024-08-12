use std::io::Cursor;

use rlox::run_file;

#[test]
fn class_1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    run_file(
        "tests/samples/class_1.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "Crunch crunch crunch!\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}

#[test]
fn class_2() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let mut writer = Cursor::new(Vec::<u8>::new());
    run_file(
        "tests/samples/class_2.txt",
        &mut writer,
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "The German chocolate cake is delicious!\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}

#[test]
fn class_3() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let mut writer = Cursor::new(Vec::<u8>::new());
    run_file(
        "tests/samples/class_3.txt",
        &mut writer,
        &mut had_error,
        &mut had_runtime_error,
        false,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    // return nothing
    assert_eq!(string, "");
    assert!(!had_error);
    assert!(!had_runtime_error);
}
