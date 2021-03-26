#[test]
fn choose_client() {
    use crate::ChooseClient;
    #[cfg(feature = "tmux_2_6")]
    use crate::TargetPane;
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Put a pane into client mode, allowing a client to be selected interactively from a list
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.7:
    // ```text
    // tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^2.6:
    // ```text
    // tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux choose-client [-F format] [-t target-window] [template]
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux choose-client  [-t target-window] [template]
    // ```

    #[cfg(feature = "tmux_2_6")]
    let target_pane = TargetPane::Raw("4").to_string();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    let target_window = TargetWindow::Raw("4").to_string();

    let mut choose_client = ChooseClient::new();
    #[cfg(feature = "tmux_2_6")]
    choose_client.without_preview();
    #[cfg(feature = "tmux_3_1")]
    choose_client.reverse_sort_order();
    #[cfg(feature = "tmux_3_1")]
    choose_client.zoom();
    #[cfg(feature = "tmux_1_7")]
    choose_client.format("1");
    #[cfg(feature = "tmux_2_6")]
    choose_client.filter("2");
    #[cfg(feature = "tmux_2_6")]
    choose_client.sort_order("3");
    #[cfg(feature = "tmux_2_6")]
    choose_client.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    choose_client.target_window(&target_window);
    #[cfg(feature = "tmux_1_0")]
    choose_client.template("5");

    let cmd = "choose-client";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_6")]
    s.push("-N");
    #[cfg(feature = "tmux_3_1")]
    s.push("-r");
    #[cfg(feature = "tmux_3_1")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_1_0")]
    s.push("5");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(choose_client.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(choose_client.0.bin_args, None);
    assert_eq!(choose_client.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(choose_client.0.cmd_args, Some(s));
}
