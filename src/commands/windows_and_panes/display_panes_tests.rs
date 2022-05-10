#[test]
fn display_panes() {
    use crate::DisplayPanes;
    use std::borrow::Cow;

    // Display a visible indicator of each pane shown by `target-client`
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux display-panes [-bN] [-d duration] [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^2.9:
    // ```text
    // tmux display-panes [-b] [-d duration] [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux display-panes [-d duration] [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // tmux display-panes [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux display-panes [-t target-client]
    // (alias: displayp)
    // ```
    let mut display_panes = DisplayPanes::new();
    #[cfg(feature = "tmux_2_9")]
    display_panes.not_block();
    #[cfg(feature = "tmux_3_2")]
    display_panes.ignore_keys();
    #[cfg(feature = "tmux_2_6")]
    display_panes.duration("1");
    #[cfg(feature = "tmux_1_0")]
    display_panes.target_client("2");
    #[cfg(feature = "tmux_2_3")]
    display_panes.template("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "displayp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    s.push("-b");
    #[cfg(feature = "tmux_3_2")]
    s.push("-N");
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_3")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_panes = display_panes.build().to_vec();

    assert_eq!(display_panes, s);
}
