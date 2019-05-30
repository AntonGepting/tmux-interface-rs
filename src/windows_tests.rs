#[test]
fn windows_parse() {
    use crate::Windows;

    let windows = Windows::parse("1559064235'0'0'0'''1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2'c3bd,177x64,0,0,0'177'0\n1559064235'0'0'0'''1'64'@1'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2'c3bd,177x64,0,0,0'177'0").unwrap();
    assert_eq!(windows[0].id, 0);
    assert_eq!(windows[1].id, 1);
}
