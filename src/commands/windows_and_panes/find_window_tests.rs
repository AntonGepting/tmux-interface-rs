// auto-generated file
//

// Search for the fnmatch(3) pattern `match-string` in window names,
// titles, and visible content (but not history)
//
// # Manual
//
// tmux >=3.2:
// ```text
// find-window [-iCNrTZ] [-t target-pane] match-string
// (alias: findw)
//
// tmux >=3.0a:
// ```text
// find-window [-rCNTZ] [-t target-pane] match-string
// (alias: findw)
//
// tmux >=2.9:
// ```text
// find-window [-CNTZ] [-t target-pane] match-string
// (alias: findw)
//
// tmux >=2.6:
// ```text
// find-window [-CNT] [-t target-pane] match-string
// (alias: findw)
//
// tmux >=1.7:
// ```text
// find-window [-CNT] [-F format] [-t target-pane] match-string
// (alias: findw)
//
// tmux >=0.8:
// ```text
// find-window [-t target-pane] match-string
// (alias: findw)
// ```
#[test]
fn find_window() {
    use crate::FindWindow;
    use std::borrow::Cow;

    let find_window = FindWindow::new();
    // `[-i]`
    #[cfg(feature = "tmux_3_2")]
    let find_window = find_window.ignore_case();

    // `[-C]`
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window.only_visible();

    // `[-N]`
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window.only_name();

    // `[-r]`
    #[cfg(feature = "tmux_3_0a")]
    let find_window = find_window.regex();

    // `[-T]`
    #[cfg(feature = "tmux_1_7")]
    let find_window = find_window.only_title();

    // `[-Z]`
    #[cfg(feature = "tmux_2_9")]
    let find_window = find_window.zoom();

    // `[-F format]`
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    let find_window = find_window.format("1");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
    let find_window = find_window.target_window("2");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    let find_window = find_window.target_pane("3");

    // `[match-string]`
    #[cfg(feature = "tmux_0_8")]
    let find_window = find_window.match_string("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "find-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "findw";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    v.push("-i");
    #[cfg(feature = "tmux_1_7")]
    v.push("-C");
    #[cfg(feature = "tmux_1_7")]
    v.push("-N");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-r");
    #[cfg(feature = "tmux_1_7")]
    v.push("-T");
    #[cfg(feature = "tmux_2_9")]
    v.push("-Z");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let find_window = find_window.build().to_vec();

    assert_eq!(find_window, v);
}
