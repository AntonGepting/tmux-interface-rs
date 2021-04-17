#[test]
fn to_string() {
    use crate::format::variable::Variable;

    let v = Variable::WindowActive.to_string();
    assert_eq!(v, "#{window_active}");
}
