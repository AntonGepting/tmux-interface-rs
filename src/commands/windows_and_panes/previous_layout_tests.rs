#[test]
fn previous_layout() {
    use crate::{PreviousLayout, TargetWindow};
    use std::borrow::Cow;

    // Move to the previous layout in the session
    //
    // # Manual
    //
    // tmux ^1.3:
    // ```text
    // previous-layout [-t target-window]
    // (alias: prevl)
    // ```

    let target_window = TargetWindow::Raw("1").to_string();

    let previous_layout = PreviousLayout::new();
    #[cfg(feature = "tmux_1_3")]
    let previous_layout = previous_layout.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prevl";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let previous_layout = previous_layout.build().to_vec();

    assert_eq!(previous_layout, s);
}
