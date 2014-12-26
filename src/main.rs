// This should fail to compile if any of these code examples are wrong.

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // Doing the string conversions all in main so that the lifetimes can be inferred.

    let s = "I am a &str.";

    // &str => String
    let st: String = s.to_string();

    // String => &str
    let str: &str = st.as_slice();
}

// int ---------------------------------------------------------------
#[allow(dead_code)]
fn int_to_uint(x: int) -> uint {
    x as uint
}

#[test]
fn test_int_to_uint_happy() {
    let x = 3i;
    let y = 3u;
    assert_eq!(y, int_to_uint(x));
}

#[allow(dead_code)]
fn int_to_string(x: int) -> String {
    x.to_string()
}

#[test]
fn test_int_to_string_happy() {
    let x = 3i;
    let y = "3";
    assert_eq!(y, int_to_string(x));
}

#[allow(dead_code)]
fn int_to_char(x: int) -> char {
    std::char::from_digit(x.to_uint().unwrap(), 10).unwrap()
}

#[test]
fn test_int_to_char_happy() {
    let x = 3i;
    let y = '3';
    assert_eq!(y, int_to_char(x));
}

// uint ---------------------------------------------------------------

#[allow(dead_code)]
fn uint_to_int(x: uint) -> int {
    x as int
}

#[test]
fn test_uint_to_int_happy() {
    let x = 3u;
    let y = 3i;
    assert_eq!(y, uint_to_int(x));
}

#[allow(dead_code)]
fn uint_to_string(x: uint) -> String {
    x.to_string()
}

#[test]
fn test_uint_to_string_happy() {
    let x = 3u;
    let y = "3";
    assert_eq!(y, uint_to_string(x));
}

#[allow(dead_code)]
fn uint_to_char(x: uint) -> char {
    std::char::from_digit(x, 10).unwrap()
}

#[test]
fn test_uint_to_char_happy() {
    let x = 3u;
    let y = '3';
    assert_eq!(y, uint_to_char(x));
}

// String ---------------------------------------------------------------

#[allow(dead_code)]
fn string_to_int(x: String) -> int {
    x.parse::<int>().unwrap()
}

#[test]
fn test_string_to_int_happy() {
    let x = "3".to_string();
    let y = 3i;
    assert_eq!(y, string_to_int(x));
}

#[allow(dead_code)]
fn string_to_uint(x: String) -> uint {
    x.parse::<uint>().unwrap()
}

#[test]
fn test_string_to_uint_happy() {
    let x = "3".to_string();
    let y = 3u;
    assert_eq!(y, string_to_uint(x));
}

#[allow(dead_code)]
fn string_to_char(x: String) -> char {
    x.char_at(0)
}

#[test]
fn test_string_to_char_happy() {
    let x = "3".to_string();
    let y = '3';
    assert_eq!(y, string_to_char(x));
}

// char ---------------------------------------------------------------

#[allow(dead_code)]
fn char_to_int(x: char) -> int {
    Char::to_digit(x, 10).unwrap().to_int().unwrap()
}

#[test]
fn test_char_to_int_happy() {
    let x = '3';
    let y = 3i;
    assert_eq!(y, char_to_int(x));
}

#[allow(dead_code)]
fn char_to_uint(x: char) -> uint {
    Char::to_digit(x, 10).unwrap()
}

#[test]
fn test_char_to_uint_happy() {
    let x = '3';
    let y = 3u;
    assert_eq!(y, char_to_uint(x));
}

#[allow(dead_code)]
fn char_to_string(x: char) -> String {
    x.to_string()
}

#[test]
fn test_char_to_string_happy() {
    let x = '3';
    let y = "3";
    assert_eq!(y, char_to_string(x));
}
