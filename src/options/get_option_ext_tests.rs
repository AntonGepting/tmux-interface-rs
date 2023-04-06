use crate::GetOptionExt;

struct GetOptionExtTest;

impl GetOptionExt for GetOptionExtTest {}

#[test]
fn get_option_ext_tests() {
    let target = ":";
    let name = "option-name";

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    let s = GetOptionExtTest::get_ext(Some(target), name).to_string();

    let origin = format!("{} -t {} {}", cmd, target, name);

    assert_eq!(origin, s);
}
