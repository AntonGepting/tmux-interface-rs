#[test]
fn display_menu() {
    use crate::{DisplayMenu, TargetPane};
    use std::borrow::Cow;

    // Structure for displaying a menu on target-client
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux display-menu [-c target-client] [-t target-pane] [-T title]
    // [-x position] [-y position] name key command ...
    // ```
    let target_pane = TargetPane::Raw("2").to_string();

    let mut display_menu = DisplayMenu::new();
    #[cfg(feature = "tmux_3_2")]
    display_menu.not_close();
    #[cfg(feature = "tmux_3_0")]
    display_menu.target_client("1");
    #[cfg(feature = "tmux_3_0")]
    display_menu.target_pane(&target_pane);
    #[cfg(feature = "tmux_3_0")]
    display_menu.title("3");
    #[cfg(feature = "tmux_3_0")]
    display_menu.x(4);
    #[cfg(feature = "tmux_3_0")]
    display_menu.y(5);
    #[cfg(feature = "tmux_3_0")]
    display_menu.name("6");
    #[cfg(feature = "tmux_3_0")]
    display_menu.key("7");
    #[cfg(feature = "tmux_3_0")]
    display_menu.command("8");

    let cmd = "display-menu";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-O");
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-T", "3"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-x", "4"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-y", "5"]);
    #[cfg(feature = "tmux_3_0")]
    s.push("6");
    #[cfg(feature = "tmux_3_0")]
    s.push("7");
    #[cfg(feature = "tmux_3_0")]
    s.push("8");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_menu = display_menu.build().to_vec();

    assert_eq!(display_menu, s);
}
