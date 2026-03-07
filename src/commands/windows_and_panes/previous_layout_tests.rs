// auto-generated file
//

// Move to the previous layout in the session
//
// # Manual
//
// tmux >=1.5:
// ```text
// previous-layout [-t target-window]
// (alias: prevl)
// ```
#[test]
fn previous_layout() {
    use crate::PreviousLayout;
    use std::borrow::Cow;

    let previous_layout = PreviousLayout::new();
    // `[-t target-window]`
    #[cfg(feature = "tmux_1_5")]
    let previous_layout = previous_layout.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prevl";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let previous_layout = previous_layout.build().to_vec();

    assert_eq!(previous_layout, v);
}
