#[test]
fn status_keys() {
    use crate::StatusKeys;

    assert_eq!(StatusKeys::Vi.to_string(), "vi");
    assert_eq!(StatusKeys::Emacs.to_string(), "emacs");
}
