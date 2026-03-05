// auto-generated file
//

// Send a key or keys to a window
//
// # Manual
//
// tmux >=3.4:
// ```text
// send-keys [-FHKlMRX] [-c target-client] [-N repeat-count] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=3.1:
// ```text
// send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=3.0a:
// ```text
// send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=2.4:
// ```text
// send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=2.1:
// ```text
// send-keys [-lMR] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=1.7:
// ```text
// send-keys [-lR] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=1.6:
// ```text
// send-keys [-R] [-t target-pane] key ...
// (alias: send)
// ```
//
// tmux >=0.8:
// ```text
// send-keys [-t target-window] key ...
// (alias: send)
// ```
#[test]
fn send_keys() {
    use crate::SendKeys;
    use std::borrow::Cow;

    let send_keys = SendKeys::new();
    // `[-F]`
    #[cfg(feature = "tmux_3_1")]
    let send_keys = send_keys.expand_formats();

    // `[-H]`
    #[cfg(feature = "tmux_3_0a")]
    let send_keys = send_keys.hex();

    // `[-K]`
    #[cfg(feature = "tmux_3_4")]
    let send_keys = send_keys.client();

    // `[-l]`
    #[cfg(feature = "tmux_1_7")]
    let send_keys = send_keys.disable_lookup();

    // `[-M]`
    #[cfg(feature = "tmux_2_1")]
    let send_keys = send_keys.mouse_event();

    // `[-R]`
    #[cfg(feature = "tmux_1_7")]
    let send_keys = send_keys.copy_mode();

    // `[-X]`
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys.reset();

    // `[-N repeat-count]`
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys.repeat_count(1);

    // `[-c target-client]`
    #[cfg(feature = "tmux_3_4")]
    let send_keys = send_keys.target_client("2");

    // `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    let send_keys = send_keys.target_window("3");

    // `[-t target-pane]`
    #[cfg(feature = "tmux_2_4")]
    let send_keys = send_keys.target_pane("4");

    // `[key]`
    #[cfg(feature = "tmux_0_8")]
    let send_keys = send_keys.key("5").key("6");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "send-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "send";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_1")]
    v.push("-F");
    #[cfg(feature = "tmux_3_0a")]
    v.push("-H");
    #[cfg(feature = "tmux_3_4")]
    v.push("-K");
    #[cfg(feature = "tmux_1_7")]
    v.push("-l");
    #[cfg(feature = "tmux_2_1")]
    v.push("-M");
    #[cfg(feature = "tmux_1_7")]
    v.push("-R");
    #[cfg(feature = "tmux_2_4")]
    v.push("-X");
    #[cfg(feature = "tmux_2_4")]
    v.extend_from_slice(&["-N", "1"]);
    #[cfg(feature = "tmux_3_4")]
    v.extend_from_slice(&["-c", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_2_4")]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("5");
    #[cfg(feature = "tmux_0_8")]
    v.push("6");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let send_keys = send_keys.build().to_vec();

    assert_eq!(send_keys, v);
}
