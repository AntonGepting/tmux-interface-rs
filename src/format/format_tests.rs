#[test]
fn to_string() {
    use crate::format::format::Format;
    use crate::format::variable::Variable;

    let mut f = Format::new();
    f.push(Variable::WindowActive);
    f.window_active().window_active();
    let f_str = f.to_string();
    dbg!(&f_str);
    //assert_eq!(f, "")
}

#[test]
fn from_string() {
    use crate::format::format::Format;
    use crate::format::variable::Variable;
    use crate::format::variable_output::VariableOutput;

    let f_str = "1";
    let f = VariableOutput::from_string_ext(f_str, &Variable::WindowActive);
    dbg!(f);
}

#[test]
fn multiple_from_string() {
    use crate::format::format::Format;
    use crate::format::format_output::FormatOutput;
    use crate::format::variable::Variable;
    use crate::format::variable_output::VariableOutput;

    let mut f = Format::new();
    f.push(Variable::WindowActive);
    f.window_active().window_active();

    let f_str = "1 1";
    let f2 = FormatOutput::from_string_ext(f_str, f);
    dbg!(f2);
}
