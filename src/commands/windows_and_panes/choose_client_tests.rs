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
    // tmux ^3.2:
    // ```text
    // tmux choose-client [-NrZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
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
    let target_pane = TargetPane::Raw("5").to_string();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    let target_window = TargetWindow::Raw("5").to_string();

    let choose_client = ChooseClient::new();
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.without_preview();
    #[cfg(feature = "tmux_3_1")]
    let choose_client = choose_client.reverse_sort_order();
    #[cfg(feature = "tmux_3_1")]
    let choose_client = choose_client.zoom();
    #[cfg(feature = "tmux_1_7")]
    let choose_client = choose_client.format("1");
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.filter("2");
    #[cfg(feature = "tmux_3_2")]
    let choose_client = choose_client.key_format("3");
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.sort_order("4");
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    let choose_client = choose_client.target_window(&target_window);
    #[cfg(feature = "tmux_1_0")]
    let choose_client = choose_client.template("6");

    let cmd = "choose-client";

    let mut s = Vec::new();
    s.push(cmd);
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
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "4"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_1_0")]
    s.push("6");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let choose_client = choose_client.build().to_vec();

    assert_eq!(choose_client, s);
}
