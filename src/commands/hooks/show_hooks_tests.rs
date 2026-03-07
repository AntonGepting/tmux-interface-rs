// auto-generated file
//

// Shows hooks
//
// # Manual
//
// tmux >=3.2:
// ```text
// show-hooks [-gpw] [-t target-session]
// ```
//
// tmux >=2.2:
// ```text
// show-hooks [-g] [-t target-session]
// ```
#[test]
fn show_hooks() {
    use crate::ShowHooks;
    use std::borrow::Cow;

    let show_hooks = ShowHooks::new();
    // `[-g]`
    #[cfg(feature = "tmux_2_2")]
    let show_hooks = show_hooks.global();

    // `[-p]`
    #[cfg(feature = "tmux_3_2")]
    let show_hooks = show_hooks.pane();

    // `[-w]`
    #[cfg(feature = "tmux_3_2")]
    let show_hooks = show_hooks.window();

    // `[-t target-session]`
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
    let show_hooks = show_hooks.target_session("1");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_3_2")]
    let show_hooks = show_hooks.target_pane("2");

    let cmd = "show-hooks";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_2")]
    v.push("-g");
    #[cfg(feature = "tmux_3_2")]
    v.push("-p");
    #[cfg(feature = "tmux_3_2")]
    v.push("-w");
    #[cfg(all(feature = "tmux_2_2", not(feature = "tmux_3_2")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-t", "2"]);
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let show_hooks = show_hooks.build().to_vec();

    assert_eq!(show_hooks, v);
}
