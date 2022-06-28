#[test]
fn list_keys() {
    use crate::ListKeys;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // list-keys [-1aN] [-P prefix-string -T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // list-keys [-T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // list-keys [-t mode-table] [-T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-keys [-t key-table]
    // (alias: lsk)
    // ```
    let list_keys = ListKeys::new();
    #[cfg(feature = "tmux_2_4")]
    let list_keys = list_keys.first();
    #[cfg(feature = "tmux_2_4")]
    let list_keys = list_keys.command();
    #[cfg(feature = "tmux_2_4")]
    let list_keys = list_keys.with_notes();
    #[cfg(feature = "tmux_3_1")]
    let list_keys = list_keys.prefix_string("1");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    let list_keys = list_keys.mode_table("2");
    #[cfg(feature = "tmux_0_8")]
    let list_keys = list_keys.key_table("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsk";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_keys = list_keys.build().to_vec();

    assert_eq!(list_keys, s);
}
