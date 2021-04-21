#[test]
fn from_string() {
    use crate::format::variable_output::VariableOutput;

    let mut f: Option<bool> = None;
    let f_str = "1";
    VariableOutput::from_string_ext(f_str, &mut VariableOutput::WindowActive(&mut f));
    assert_eq!(f, Some(true));
}
