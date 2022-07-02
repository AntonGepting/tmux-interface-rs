#[test]
fn display_popup() {
    #[cfg(feature = "tmux_3_3")]
    use crate::commands::PopupBorderLinesType;
    use crate::commands::Size;
    use crate::{DisplayPopup, TargetPane};
    use std::borrow::Cow;

    // Structure for displaying a menu on target-client
    //
    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
    // [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
    // [-y position] [shell-command]
    // ```
    //
    // tmux ^3.2:
    // ```text
    // display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
    // [-w width] [-x position] [-y position] [shell-command]
    // (alias: popup)
    // ```
    let target_pane = TargetPane::Raw("8").to_string();

    let display_popup = DisplayPopup::new();
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.no_border();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close_on_exit();
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.close_on_success();
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.border_lines(PopupBorderLinesType::NoBorder);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.target_client("1");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.start_directory("2");
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.environment("3", "4");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.height(Size::Size(5));
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.style("6");
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.border_style("7");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.target_pane(&target_pane);
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup.title("9");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.width(Size::Size(10));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.x(Size::Size(11));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.y(Size::Size(12));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup.shell_command("13");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-popup";
    #[cfg(feature = "cmd_alias")]
    let cmd = "popup";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    s.push("-B");
    #[cfg(feature = "tmux_3_2")]
    s.push("-C");
    #[cfg(feature = "tmux_3_2")]
    s.push("-E");
    #[cfg(feature = "tmux_3_2")]
    s.push("-EE");
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-b", "none"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "2"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-e", "3=4"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-h", "5"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-s", "6"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-S", "7"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "8"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-T", "9"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-w", "10"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-x", "11"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-y", "12"]);
    #[cfg(feature = "tmux_3_2")]
    s.push("13");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_popup = display_popup.build().to_vec();

    assert_eq!(display_popup, s);
}
