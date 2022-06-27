#[test]
fn display_popup() {
    use crate::commands::common::pane_size::Size;
    use crate::{DisplayPopup, TargetPane};
    use std::borrow::Cow;

    // Structure for displaying a menu on target-client
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
    // [-w width] [-x position] [-y position] [shell-command]
    // (alias: popup)
    // ```
    let target_pane = TargetPane::Raw("4").to_string();

    let display_popup = DisplayPopup::new();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close_on_exit();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close_on_success();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.target_client("1");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.start_directory("2");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.height(Size::Size(3));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.target_pane(&target_pane);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.width(Size::Size(5));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.x(Size::Size(6));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.y(Size::Size(7));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.shell_command("8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-popup";
    #[cfg(feature = "cmd_alias")]
    let cmd = "popup";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-C");
    #[cfg(feature = "tmux_3_2")]
    s.push("-E");
    #[cfg(feature = "tmux_3_2")]
    s.push("-EE");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-h", "3"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-w", "5"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-x", "6"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-y", "7"]);
    #[cfg(feature = "tmux_3_2")]
    s.push("8");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_popup = display_popup.build().to_vec();

    assert_eq!(display_popup, s);
}
