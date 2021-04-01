#[test]
fn send_keys() {
    use crate::{SendKeys, TargetPaneExt};
    use std::borrow::Cow;

    // Structure
    //
    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux send-keys [-FHlMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^3.0:
    // ```text
    // tmux send-keys [-HlMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux send-keys [-lMRX] [-N repeat-count] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux send-keys [-lMR] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^1.7:
    // ```text
    // tmux send-keys [-lR] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // tmux send-keys [-R] [-t target-pane] key ...
    // (alias: send)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux send-keys [-t target-window] key ...
    // (alias: send)
    // ```
    let target_pane = TargetPaneExt::raw("2").to_string();

    let mut send_keys = SendKeys::new();
    #[cfg(feature = "tmux_3_1")]
    send_keys.expand_formats();
    #[cfg(feature = "tmux_3_0")]
    send_keys.hex();
    #[cfg(feature = "tmux_1_7")]
    send_keys.disable_lookup();
    #[cfg(feature = "tmux_2_1")]
    send_keys.mouse_event();
    #[cfg(feature = "tmux_1_6")]
    send_keys.copy_mode();
    #[cfg(feature = "tmux_2_4")]
    send_keys.reset();
    #[cfg(feature = "tmux_2_4")]
    send_keys.repeat_count(1);
    #[cfg(feature = "tmux_1_6")]
    send_keys.target_pane(&target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    send_keys.target_window(&target_pane);
    #[cfg(feature = "tmux_0_8")]
    send_keys.key("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "send-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "send";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_3_1")]
    s.push("-F");
    #[cfg(feature = "tmux_3_0")]
    s.push("-H");
    #[cfg(feature = "tmux_1_7")]
    s.push("-l");
    #[cfg(feature = "tmux_2_1")]
    s.push("-M");
    #[cfg(feature = "tmux_1_6")]
    s.push("-R");
    #[cfg(feature = "tmux_2_4")]
    s.push("-X");
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-N", "1"]);
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    s.extend_from_slice(&["-t", "2"]);
    s.push("3");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(send_keys.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(send_keys.0.bin_args, None);
    assert_eq!(send_keys.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(send_keys.0.cmd_args, Some(s));
}
