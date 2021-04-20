#[test]
fn to_string() {
    use crate::format::format::Format;
    use crate::format::variable::Variable;

    let mut f = Format::new();
    f.push(Variable::WindowActive);
    assert_eq!(f.to_string(), "#{window_active}")
}
