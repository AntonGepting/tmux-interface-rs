#[test]
fn from_string() {
    use crate::format::format::Format;
    use crate::format::variable::Variable;

    let mut f: Option<bool> = None;
    let f_str = "1";
    Variable::from_string_ext(f_str, &mut Variable::WindowActive(&mut f));
    dbg!(f);
}

#[test]
fn multiple_from_string() {
    use crate::format::format::Format;
    use crate::format::variable::Variable;

    let mut c: Option<bool> = None;
    let mut d: Option<bool> = None;

    let mut f = Format::new();
    f.push(Variable::WindowActive(&mut c));
    f.window_active(&mut d);

    let f_str = "1\'1";
    Format::from_string_ext(f_str, &mut f);
    dbg!(c);
    dbg!(d);
}

#[test]
fn struct_from_string() {
    use crate::format::format::Format;
    use crate::format::variable::Variable;

    #[derive(Debug, Default)]
    struct FormatStruct {
        field1: Option<bool>,
        field2: Option<bool>,
    };

    let mut format_struct = FormatStruct::default();

    let mut f = Format::new();
    f.push(Variable::WindowActive(&mut format_struct.field1));
    f.window_active(&mut format_struct.field2);

    let f_str = "1\'1";
    Format::from_string_ext(f_str, &mut f);
    dbg!(format_struct);
}
