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
    let list_panes = ListPanes::new();
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes.all();
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes.session();
    #[cfg(feature = "tmux_1_6")]
    let list_panes = list_panes.format("1");
    #[cfg(feature = "tmux_0_8")]
    let list_panes = list_panes.target("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(feature = "tmux_1_5")]
    s.push("-s");
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_panes = list_panes.build().to_vec();

    assert_eq!(list_panes, s);
}
