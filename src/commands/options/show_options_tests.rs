#[test]
fn show_options() {
    use crate::{ShowOptions, TargetPane};
    use std::borrow::Cow;

    // Structure for showing options
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // tmux show-options [-AgHpqsvw] [-t target-pane] [option]
    // (alias: show)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux show-options [-gqsvw] [-t target-session | target-window] [option]
    // (alias: show)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux show-options [-gsw] [-t target-session | target-window] [option]
    // (alias: show)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux show-options [-gsw] [-t target-session | target-window]
    // (alias: show)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux show-options [-t target-session]
    // (alias: show)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux show-options [-t target-session] option value
    // (alias: show)
    // ```

    let target_pane = TargetPane::Raw("1").to_string();
    let mut show_options = ShowOptions::new();
    #[cfg(feature = "tmux_3_0")]
    show_options.include_inherited();
    #[cfg(feature = "tmux_1_2")]
    show_options.global();
    #[cfg(feature = "tmux_3_0")]
    show_options.hooks();
    #[cfg(feature = "tmux_3_0")]
    show_options.pane();
    #[cfg(feature = "tmux_1_8")]
    show_options.quiet();
    #[cfg(feature = "tmux_1_2")]
    show_options.server();
    #[cfg(feature = "tmux_1_8")]
    show_options.value();
    #[cfg(feature = "tmux_1_2")]
    show_options.window();
    show_options.target(&target_pane);
    #[cfg(feature = "tmux_1_7")]
    show_options.option("2");

    let mut s = Vec::new();
    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    #[cfg(feature = "tmux_3_0")]
    s.push("-A");
    #[cfg(feature = "tmux_1_2")]
    s.push("-g");
    #[cfg(feature = "tmux_3_0")]
    s.push("-H");
    #[cfg(feature = "tmux_3_0")]
    s.push("-p");
    #[cfg(feature = "tmux_1_8")]
    s.push("-q");
    #[cfg(feature = "tmux_1_2")]
    s.push("-s");
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_1_2")]
    s.push("-w");
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(show_options.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(show_options.0.bin_args, None);
    assert_eq!(show_options.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(show_options.0.cmd_args, Some(s));
}
