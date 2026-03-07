// auto-generated file
//

// List panes on the server
//
// # Manual
//
// tmux >=3.2:
// ```text
// list-panes [-as] [-F format] [-f filter] [-t target]
// (alias: lsp)
// ```
//
// tmux >=1.6:
// ```text
// list-panes [-as] [-F format] [-t target]
// (alias: lsp)
// ```
//
// tmux >=1.5:
// ```text
// list-panes [-as] [-t target]
// (alias: lsp)
// ```
#[test]
fn list_panes() {
    use crate::ListPanes;
    use std::borrow::Cow;

    let list_panes = ListPanes::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes.all();

    // `[-s]`
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes.session();

    // `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    let list_panes = list_panes.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    let list_panes = list_panes.filter("2");

    // `[-t target]`
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes.target("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsp";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_1_5")]
    v.push("-s");
    #[cfg(feature = "tmux_1_6")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_1_5")]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let list_panes = list_panes.build().to_vec();

    assert_eq!(list_panes, v);
}
