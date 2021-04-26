use fmt_iter::{repeat, FmtIter};
use pipe_trait::Pipe;

#[test]
fn display_repeat_char() {
    assert_eq!(repeat('x', 3).to_string(), "xxx");
}

#[test]
fn display_slice_char() {
    let actual = ['a', 'b', 'c'].pipe_as_ref(FmtIter::from).to_string();
    let expected = "abc";
    assert_eq!(actual, expected);
}

#[test]
fn display_repeat_string() {
    assert_eq!(repeat("abc", 3).to_string(), "abcabcabc");
}

#[test]
fn display_slice_string() {
    let actual = ["abc", "def", "ghi"].pipe_as_ref(FmtIter::from).to_string();
    let expected = "abcdefghi";
    assert_eq!(actual, expected);
}

#[test]
fn iter() {
    #[allow(clippy::infinite_iter)] // false negative in rust-analyzer
    let actual: Vec<_> = repeat('x', 5).collect();
    let expected = ['x', 'x', 'x', 'x', 'x'];
    assert_eq!(actual, expected);
}

#[test]
fn exact_size_iter() {
    let actual = FmtIter::from(&[0, 1, 2, 3, 4]).len();
    let expected = 5;
    assert_eq!(actual, expected);
}

#[test]
fn repeat_len() {
    let actual = repeat('x', 5).len();
    let expected = 5;
    assert_eq!(actual, expected);
}
