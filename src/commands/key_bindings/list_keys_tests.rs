#[test]
fn list_keys() {
    use crate::ListKeys;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // tmux list-keys [-1aN] [-P prefix-string -T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux list-keys [-T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux list-keys [-t mode-table] [-T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-keys [-t key-table]
    // (alias: lsk)
    // ```
    let mut list_keys = ListKeys::new();
    #[cfg(feature = "tmux_2_4")]
    list_keys.first();
    #[cfg(feature = "tmux_2_4")]
    list_keys.command();
    #[cfg(feature = "tmux_2_4")]
    list_keys.with_notes();
    #[cfg(feature = "tmux_3_1")]
    list_keys.prefix_string("1");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    list_keys.mode_table("2");
    #[cfg(feature = "tmux_0_8")]
    list_keys.key_table("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsk";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_4")]
    s.push("-1");
    #[cfg(feature = "tmux_2_4")]
    s.push("-a");
    #[cfg(feature = "tmux_2_4")]
    s.push("-N");
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-P", "1"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-T", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    s.extend_from_slice(&["-t", "3"]);
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_keys.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_keys.0.bin_args, None);
    assert_eq!(list_keys.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(list_keys.0.cmd_args, Some(s));
}
