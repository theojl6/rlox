use std::io::Cursor;

use crate::helpers::test_file;

#[test]
fn inheritance_1() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/inheritance_1.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "Fry until golden brown.\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}

#[test]
fn inheritance_2() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/inheritance_2.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(
        string,
        "Fry until golden brown.\nPipe full of custard and coat with chocolate.\n"
    );
    assert!(!had_error);
    assert!(!had_runtime_error);
}

#[test]
fn inheritance_3() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let writer = Cursor::new(Vec::<u8>::new());
    let writer = test_file(
        "tests/samples/inheritance_3.txt",
        writer,
        &mut had_error,
        &mut had_runtime_error,
    );
    let string = String::from_utf8((&writer.get_ref()).to_vec()).expect("Found invalid UTF-8");
    assert_eq!(string, "A method\n");
    assert!(!had_error);
    assert!(!had_runtime_error);
}
