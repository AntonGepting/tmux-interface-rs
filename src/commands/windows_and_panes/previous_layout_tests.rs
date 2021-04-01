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
    // tmux previous-layout [-t target-window]
    // (alias: prevl)
    // ```

    let target_window = TargetWindow::Raw("1").to_string();

    let mut previous_layout = PreviousLayout::new();
    #[cfg(feature = "tmux_1_3")]
    previous_layout.target_window(&target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prevl";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-t", "1"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(previous_layout.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(previous_layout.0.bin_args, None);
    assert_eq!(previous_layout.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(previous_layout.0.cmd_args, Some(s));
}
