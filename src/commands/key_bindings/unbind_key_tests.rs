#[test]
fn unbind_key() {
    use crate::UnbindKey;
    use std::borrow::Cow;

    // # Manual
    //
    // ```text
    // tmux ^2.4:
    // tmux unbind-key [-an] [-T key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux unbind-key [-acn] [-t mode-table] [-T key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // tmux unbind-key [-acn] [-t mode-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^1.4:
    // ```text
    // tmux unbind-key [-acn] [-t key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux unbind-key [-cn] [-t key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux unbind-key key
    // (alias: unbind)
    // ```
    let mut unbind_key = UnbindKey::new();
    #[cfg(feature = "tmux_1_4")]
    unbind_key.all();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    unbind_key.command_mode();
    #[cfg(feature = "tmux_1_0")]
    unbind_key.root();
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    unbind_key.mode_key("1");
    #[cfg(feature = "tmux_1_0")]
    unbind_key.key_table("2");
    #[cfg(feature = "tmux_0_8")]
    unbind_key.key("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unbind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unbind";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_4")]
    s.push("-a");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    s.push("-c");
    #[cfg(feature = "tmux_1_0")]
    s.push("-n");
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(unbind_key.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(unbind_key.0.bin_args, None);
    assert_eq!(unbind_key.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(unbind_key.0.cmd_args, Some(s));
}
