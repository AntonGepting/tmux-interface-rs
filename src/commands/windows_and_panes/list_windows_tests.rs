#[test]
fn list_windows() {
    use crate::{ListWindows, TargetSession};
    use std::borrow::Cow;

    // List windows on the server
    //
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // tmux list-windows [-a] [-F format] [-t target-session]
    // (alias: lsw)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux list-windows [-a] [-t target-session]
    // (alias: lsw)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-windows [-t target-session]
    // (alias: lsw)
    // ```
    let target_session = TargetSession::Raw("2").to_string();
    let mut list_windows = ListWindows::new();
    #[cfg(feature = "tmux_1_5")]
    list_windows.all();
    #[cfg(feature = "tmux_1_6")]
    list_windows.format("1");
    #[cfg(feature = "tmux_0_8")]
    list_windows.target_session(&target_session);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-windows";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsw";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_windows.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_windows.0.bin_args, None);
    assert_eq!(list_windows.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(list_windows.0.cmd_args, Some(s));
}
