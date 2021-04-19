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
