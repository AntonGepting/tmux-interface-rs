#[test]
fn list_panes() {
    use crate::ListPanes;
    use std::borrow::Cow;

    // List panes on the server
    //
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // tmux list-panes [-as] [-F format] [-t target]
    // (alias: lsp)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // tmux list-panes [-as] [-t target]
    // (alias: lsp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-panes [-t target]
    // (alias: lsp)
    // ```
    let mut list_panes = ListPanes::new();
    #[cfg(feature = "tmux_1_5")]
    list_panes.all();
    #[cfg(feature = "tmux_1_5")]
    list_panes.session();
    #[cfg(feature = "tmux_1_6")]
    list_panes.format("1");
    #[cfg(feature = "tmux_0_8")]
    list_panes.target("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsp";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(feature = "tmux_1_5")]
    s.push("-s");
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_panes.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_panes.0.bin_args, None);
    assert_eq!(list_panes.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(list_panes.0.cmd_args, Some(s));
}
