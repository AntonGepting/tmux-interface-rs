use crate::SetOptionExt;

struct SetOptionExtTest;

impl SetOptionExt for SetOptionExtTest {}

#[test]
fn set_option_ext_tests() {
    let target = ":";
    let name = "option-name";
    let value = "value";

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let s = SetOptionExtTest::set_ext(Some(target), name, Some(value)).to_string();

    let origin = format!("{} -t {} {} {}", cmd, target, name, value);

    assert_eq!(origin, s);
}
