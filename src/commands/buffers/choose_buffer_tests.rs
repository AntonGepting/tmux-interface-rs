#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, TargetPane};
    use std::borrow::Cow;

    // Stucture for putting a pane into buffer mode
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // choose-buffer [-NZr] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^3.1:
    // ```text
    // choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.7:
    // ```text
    // choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.6:
    // ```text
    // choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // choose-buffer [-F format] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.3:
    // ```text
    // choose-buffer [-t target-pane] [template]
    // ```
    let target_pane = TargetPane::Raw("5").to_string();

    let choose_buffer = ChooseBuffer::new();
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.no_preview();
    #[cfg(feature = "tmux_2_7")]
    let choose_buffer = choose_buffer.zoom();
    #[cfg(feature = "tmux_3_1")]
    let choose_buffer = choose_buffer.reverse_sort_order();
    #[cfg(feature = "tmux_1_7")]
    let choose_buffer = choose_buffer.format("1");
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.filter("2");
    #[cfg(feature = "tmux_3_2")]
    let choose_buffer = choose_buffer.key_format("3");
    #[cfg(feature = "tmux_2_6")]
    let choose_buffer = choose_buffer.sort_order("4");
    #[cfg(feature = "tmux_1_3")]
    let choose_buffer = choose_buffer.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_3")]
    let choose_buffer = choose_buffer.template("6");

    let cmd = "choose-buffer";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_6")]
    s.push("-N");
    #[cfg(feature = "tmux_2_7")]
    s.push("-Z");
    #[cfg(feature = "tmux_3_1")]
    s.push("-r");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "4"]);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_3")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let choose_buffer = choose_buffer.build().to_vec();

    assert_eq!(choose_buffer, s);
}
