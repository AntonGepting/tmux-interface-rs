#[test]
fn from_str() {
    use crate::format::variable::Variable;

    let mut v: Option<bool> = None;
    let v_str = "1";
    Variable::from_string_ext(v_str, &mut Variable::WindowActive(&mut v));
    assert_eq!(v, Some(true));
}
