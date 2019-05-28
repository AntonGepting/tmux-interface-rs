#[test]
fn layout_parse() {
    use regex::Regex;

    let regex = Regex::new(r"^'([\w,]*)'$").unwrap();
    assert!(regex.is_match("'c3bf,177x64,0,0,2'"));
}


#[test]
fn flags_parse() {
    use regex::Regex;

    let regex = Regex::new(r"^'([\w\*]*)'$").unwrap();
    assert!(regex.is_match("'*'"));
}



#[test]
fn parse() {
    use crate::Window;

    let window = Window::parse("1557947146'0'0'0''*'1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2'c3bd,177x64,0,0,0'177'0").unwrap();
    assert_eq!(window.name, "bash");
    assert_eq!(window.id, 0);
    // FIXME: name with dots
    //let window = Window::from_str("1557947146 0 0 0 @0 1 python3.7 0").unwrap();
    //assert_eq!(window.name, "asdf");
    //assert_eq!(window.id, 0);
}
