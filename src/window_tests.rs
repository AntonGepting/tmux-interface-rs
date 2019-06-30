#[test]
fn layout_parse() {
    use regex::Regex;
    let regex = Regex::new(r"^'([\w,\[\]]*)?'$").unwrap();
    assert!(regex.is_match("'c3bf,177x64,0,0,2'"));
    assert!(regex.is_match("'8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'"));
    assert!(regex.is_match("'8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'"));
    assert!(regex.is_match("'8b65,177x64,0,0{177x46,0,0,1,177x17,0,47,4}'"));
    assert!(regex.is_match("''"));
}


#[test]
fn flags_parse() {
    use regex::Regex;
    let regex = Regex::new(r"^'([\w\*-]*)'$").unwrap();
    assert!(regex.is_match("'*'"));
    assert!(regex.is_match("'-'"));
    assert!(regex.is_match("''"));
}


#[test]
fn parse() {
    use crate::Window;

    let window = Window::parse("1557947146'0'1'0'''*'1'64'@3'4'0'3484,177x64,0,0{88x64,0,0,3,88x64,89,0,18}'0'bash'''2'0'0''3484,177x64,0,0{88x64,0,0,3,88x64,89,0,18}'177'0").unwrap();
    assert_eq!(window.id, Some(3));

    let window = Window::parse("1557947146'0'0'0'''*'1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2''c3bd,177x64,0,0,0'177'0").unwrap();
    assert_eq!(window.name, Some("bash".to_string()));
    assert_eq!(window.id, Some(0));

    let window = Window::parse("1557947146'0'0'0''''1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'3''c3bd,177x64,0,0,0'177'0").unwrap();
    assert_eq!(window.id, Some(0));

    let window = Window::parse("1557947146'0'0'0''''1'64'@1'2'0'8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'0'vim'''2'0'2''8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'177'0").unwrap();
    assert_eq!(window.id, Some(1));

    let window = Window::parse("1557947146'0'0'0'''-'1'64'@2'3'1'7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'0'vim'''2'0'1''7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'177'0").unwrap();
    assert_eq!(window.id, Some(2));

    let window = Window::parse("1557947146'0'1'0'''*'1'64'@4'4'0'c3c3,177x64,0,0,6'0'bash'''1'0'0''c3c3,177x64,0,0,6'177'0").unwrap();
    assert_eq!(window.id, Some(4));
    // FIXME: name with dots
    //let window = Window::from_str("1557947146 0 0 0 @0 1 python3.7 0").unwrap();
    //assert_eq!(window.name, "asdf");
    //assert_eq!(window.id, 0);
}
