// auto-generated file
//

// Structure for putting a pane into buffer mode
//
// # Manual
//
// tmux >=3.2:
// ```text
// choose-buffer [-NZr] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=3.1:
// ```text
// choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=2.7:
// ```text
// choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=2.6:
// ```text
// choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=1.7:
// ```text
// choose-buffer [-F format] [-t target-window] [template]
// ```
//
// tmux >=1.5:
// ```text
// choose-buffer [-t target-window] [template]
// ```
#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, TargetPane};
    use std::borrow::Cow;

    let target_pane = TargetPane::Raw("5").to_string();

    let choose_buffer = ChooseBuffer::new();
    /// `[-N]` - start without the preview
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.no_preview();

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_2_7")]
    let choose_buffer = choose_buffer.zoom();

    /// `[-r]` - reverses the sort order
    #[cfg(feature = "tmux_3_1")]
    let choose_buffer = choose_buffer.reverse_sort_order();

    /// `[-F format]` - specify the format for each item in the list
    #[cfg(feature = "tmux_1_7")]
    let choose_buffer = choose_buffer.format("1");

    /// `[-f filter]` - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.filter("2");

    /// `[-K key-format]` - format for each shortcut key
    #[cfg(feature = "tmux_3_2")]
    let choose_buffer = choose_buffer.key_format("3");

    /// `[-O sort-order]` - specifies the initial sort field
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.sort_order("4");

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let choose_buffer = choose_buffer.target_window("5");

    /// `[-t target-pane]` - specify the target pane
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.target_pane("6");

    /// `[template]` - specify the template
    #[cfg(feature = "tmux_1_5")]
    let choose_buffer = choose_buffer.template("7");

    let cmd = "choose-buffer";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_6")]
    v.push("-N");
    #[cfg(feature = "tmux_2_7")]
    v.push("-Z");
    #[cfg(feature = "tmux_3_1")]
    v.push("-r");
    #[cfg(feature = "tmux_1_7")]
    v.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_3_2")]
    v.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-O", "4"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    v.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_2_6")]
    v.extend_from_slice(&["-t", "6"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("7");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let choose_buffer = choose_buffer.build().to_vec();

    assert_eq!(choose_buffer, v);
}
