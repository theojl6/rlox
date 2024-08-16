use std::io::Cursor;

use crate::helpers::test_file;

#[test]
fn variables_1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/variables_1.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "1\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}

#[test]
fn variables_2() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/variables_2.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "2\n3\n4\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}
