// auto-generated file
//

// List windows on the server
//
// # Manual
//
// tmux >=3.2:
// ```text
// list-windows [-a] [-F format] [-f filter] [-t target-session]
// (alias: lsw)
// ```
//
// tmux >=1.6:
// ```text
// list-windows [-a] [-F format] [-t target-session]
// (alias: lsw)
// ```
//
// tmux >=1.5:
// ```text
// list-windows [-a] [-t target-session]
// (alias: lsw)
// ```
//
// tmux >=0.8:
// ```text
// list-windows [-t target-session]
// (alias: lsw)
// ```
#[test]
fn list_windows() {
    use crate::ListWindows;
    use std::borrow::Cow;

    let list_windows = ListWindows::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let list_windows = list_windows.all();

    // `[-F format]`
    #[cfg(feature = "tmux_1_6")]
    let list_windows = list_windows.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    let list_windows = list_windows.filter("2");

    // `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    let list_windows = list_windows.target_session("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-windows";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(feature = "tmux_1_6")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_0_8")]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let list_windows = list_windows.build().to_vec();

    assert_eq!(list_windows, v);
}
