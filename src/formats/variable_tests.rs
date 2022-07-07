#[test]
fn to_string() {
    use crate::Variable;

    let v = Variable::WindowActive;
    assert_eq!(v.to_string(), "#{window_active}");
}
