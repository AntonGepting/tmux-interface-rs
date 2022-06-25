#[test]
fn to_string() {
    use crate::formats::variable::Variable;

    let v = Variable::WindowActive;
    assert_eq!(v.to_string(), "#{window_active}");
}
