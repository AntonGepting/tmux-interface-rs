// auto-generated file
//

// Move a window to the next layout and rearrange the panes to fit
//
// # Manual
//
// tmux >=0.8:
// ```text
// next-layout [-t target-window]
// (alias: nextl)
// ```
#[test]
fn next_layout() {
    use crate::NextLayout;
    use std::borrow::Cow;

    let next_layout = NextLayout::new();
    // `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    let next_layout = next_layout.target_window("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "nextl";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "1"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let next_layout = next_layout.build().to_vec();

    assert_eq!(next_layout, v);
}
