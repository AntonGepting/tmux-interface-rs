#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, TargetPane};
    use std::borrow::Cow;

    // Stucture for putting a pane into buffer mode
    //
    // # Manual
    //
    // tmux X.X:
    // ```text
    // tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^3.1:
    // ```text
    // tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.7:
    // ```text
    // tmux choose-buffer [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux choose-buffer [-F format] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.3:
    // ```text
    // tmux choose-buffer [-t target-pane] [template]
    // ```
    let target_pane = TargetPane::Raw("4").to_string();

    let mut choose_buffer = ChooseBuffer::new();
    #[cfg(feature = "tmux_2_6")]
    choose_buffer.no_preview();
    #[cfg(feature = "tmux_2_7")]
    choose_buffer.zoom();
    #[cfg(feature = "tmux_3_1")]
    choose_buffer.reverse_sort_order();
    #[cfg(feature = "tmux_1_7")]
    choose_buffer.format("1");
    #[cfg(feature = "tmux_2_6")]
    choose_buffer.filter("2");
    #[cfg(feature = "tmux_2_6")]
    choose_buffer.sort_order("3");
    #[cfg(feature = "tmux_1_3")]
    choose_buffer.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_3")]
    choose_buffer.template("5");

    let mut s = Vec::new();
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
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "3"]);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_3")]
    s.push("5");
    let s = s.into_iter().map(|a| a.into()).collect();

    let cmd = "choose-buffer";

    assert_eq!(choose_buffer.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(choose_buffer.0.bin_args, None);
    assert_eq!(choose_buffer.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(choose_buffer.0.cmd_args, Some(s));
}
