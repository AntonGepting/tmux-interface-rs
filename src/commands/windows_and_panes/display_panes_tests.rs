// auto-generated file
//

// Display a visible indicator of each pane shown by `target-client`
//
// # Manual
//
// tmux >=3.2:
// ```text
// display-panes [-bN] [-d duration] [-t target-client] [template]
// (alias: displayp)
// ```
//
// tmux >=2.9:
// ```text
// display-panes [-b] [-d duration] [-t target-client] [template]
// (alias: displayp)
// ```
//
// tmux >=2.6:
// ```text
// display-panes [-d duration] [-t target-client] [template]
// (alias: displayp)
// ```
//
// tmux >=2.3:
// ```text
// display-panes [-t target-client] [template]
// (alias: displayp)
// ```
//
// tmux >=1.5:
// ```text
// display-panes [-t target-client]
// (alias: displayp)
// ```
#[test]
fn display_panes() {
    use crate::DisplayPanes;
    use std::borrow::Cow;

    let display_panes = DisplayPanes::new();
    // `[-b]`
    #[cfg(feature = "tmux_2_9")]
    let display_panes = display_panes.not_block();

    // `[-N]`
    #[cfg(feature = "tmux_3_2")]
    let display_panes = display_panes.ignore_keys();

    // `[-d duration]`
    #[cfg(feature = "tmux_2_6")]
    let display_panes = display_panes.duration("1");

    // `[-t target-client]`
    #[cfg(feature = "tmux_1_5")]
    let display_panes = display_panes.target_client("2");

    // `[template]`
    #[cfg(feature = "tmux_2_3")]
    let display_panes = display_panes.template("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "displayp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    v.push("-b");
    #[cfg(feature = "tmux_3_2")]
    v.push("-N");
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_3")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let display_panes = display_panes.build().to_vec();

    assert_eq!(display_panes, v);
}
