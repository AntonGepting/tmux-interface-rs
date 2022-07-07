#[test]
fn multiple_from_string() {
    use crate::FormatsOutput;
    use crate::VariableOutput;

    let mut c: Option<bool> = None;
    let mut d: Option<bool> = None;

    let mut f = FormatsOutput::new();
    f.push(VariableOutput::WindowActive(&mut c));
    f.window_active(&mut d);

    let f_str = "1\'1";
    FormatsOutput::from_string_ext(f_str, &mut f);
    assert_eq!((c, d), (Some(true), Some(true)));
}

#[test]
fn struct_from_string() {
    use crate::FormatsOutput;
    use crate::VariableOutput;

    #[derive(Debug, PartialEq, Default)]
    struct FormatStruct {
        field1: Option<bool>,
        field2: Option<bool>,
    }

    let mut format_struct = FormatStruct::default();

    let mut f = FormatsOutput::new();
    f.push(VariableOutput::WindowActive(&mut format_struct.field1));
    f.window_active(&mut format_struct.field2);

    let f_str = "1\'1";
    FormatsOutput::from_string_ext(f_str, &mut f);
    assert_eq!(
        format_struct,
        FormatStruct {
            field1: Some(true),
            field2: Some(true)
        }
    );
}
