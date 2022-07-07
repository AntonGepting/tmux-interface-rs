#[test]
fn to_string() {
    use crate::Formats;
    use crate::Variable;

    let mut f = Formats::new();
    f.push(Variable::WindowActive);
    assert_eq!(f.to_string(), "#{window_active}")
}
