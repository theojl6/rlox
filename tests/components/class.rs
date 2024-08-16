use crate::helpers::test_file;
use std::io::Cursor;

#[test]
fn class_1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/class_1.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
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
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/class_2.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
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
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/class_3.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    // return nothing
    assert_eq!(string, "");
    assert!(!had_error);
    assert!(!had_runtime_error);
}
