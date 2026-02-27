// auto-generated file
//

// List the global buffers.
//
// # Manual
//
// tmux >=3.2:
// ```text
// list-buffers [-F format] [-f filter]
// (alias: lsb)
// ```
//
// tmux >=1.7:
// ```text
// list-buffers [-F format]
// (alias: lsb)
// ```
//
// tmux >=1.5:
// ```text
// list-buffers
// (alias: lsb)
// ```
//
// tmux >=0.8:
// ```text
// list-buffers [-t target-session]
// (alias: lsb)
// ```
#[test]
fn list_buffers() {
    use crate::{ListBuffers, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let list_buffers = ListBuffers::new();
    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let list_buffers = list_buffers.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_3_2")]
    let list_buffers = list_buffers.filter("2");

    // `[-t target_session]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let list_buffers = list_buffers.target_session("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-buffers";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsb";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    v.extend_from_slice(&["-t", "3"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let list_buffers = list_buffers.build().to_vec();

    assert_eq!(list_buffers, v);
}
