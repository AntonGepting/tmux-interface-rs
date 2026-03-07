// auto-generated file
//

// Put a pane into client mode, allowing a client to be selected interactively from a list
//
// # Manual
//
// tmux >=3.6:
// ```text
// choose-client [-NryZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=3.2:
// ```text
// choose-client [-NrZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=3.1:
// ```text
// choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=2.7:
// ```text
// choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=2.6:
// ```text
// choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
// ```
//
// tmux >=1.7:
// ```text
// choose-client [-F format] [-t target-window] [template]
// ```
//
// tmux >=1.0:
// ```text
// choose-client  [-t target-window] [template]
// ```
#[test]
fn choose_client() {
    use crate::ChooseClient;
    use std::borrow::Cow;

    let choose_client = ChooseClient::new();
    // `[-N]`
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.without_preview();

    // `[-r]`
    #[cfg(feature = "tmux_3_1")]
    let choose_client = choose_client.reverse_sort_order();

    // `[-y]`
    #[cfg(feature = "tmux_3_6")]
    let choose_client = choose_client.disable_confirmation();

    // `[-Z]`
    #[cfg(feature = "tmux_2_7")]
    let choose_client = choose_client.zoom();

    // `[-F format]`
    #[cfg(feature = "tmux_1_7")]
    let choose_client = choose_client.format("1");

    // `[-f filter]`
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.filter("2");

    // `[-K key-format]`
    #[cfg(feature = "tmux_3_2")]
    let choose_client = choose_client.key_format("3");

    // `[-O sort-order]`
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.sort_order("4");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let choose_client = choose_client.target_window("5");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client.target_pane("6");

    // `[template]`
    #[cfg(feature = "tmux_1_5")]
    let choose_client = choose_client.template("7");

    let cmd = "choose-client";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_2_6")]
    v.push("-N");
    #[cfg(feature = "tmux_3_1")]
    v.push("-r");
    #[cfg(feature = "tmux_3_6")]
    v.push("-y");
    #[cfg(feature = "tmux_2_7")]
    v.push("-Z");
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

    let choose_client = choose_client.build().to_vec();

    assert_eq!(choose_client, v);
}
