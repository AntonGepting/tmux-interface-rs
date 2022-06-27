#[test]
fn unbind_key() {
    use crate::UnbindKey;
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux unbind-key [-anq] [-T key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^2.4:
    // ```text
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
    let unbind_key = UnbindKey::new();
    #[cfg(feature = "tmux_1_4")]
    let unbind_key = unbind_key.all();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let unbind_key = unbind_key.command_mode();
    #[cfg(feature = "tmux_1_0")]
    let unbind_key = unbind_key.root();
    #[cfg(feature = "tmux_3_2")]
    let unbind_key = unbind_key.quiet();
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    let unbind_key = unbind_key.mode_key("1");
    #[cfg(feature = "tmux_1_0")]
    let unbind_key = unbind_key.key_table("2");
    #[cfg(feature = "tmux_0_8")]
    let unbind_key = unbind_key.key("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unbind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unbind";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_4")]
    s.push("-a");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    s.push("-c");
    #[cfg(feature = "tmux_1_0")]
    s.push("-n");
    #[cfg(feature = "tmux_3_2")]
    s.push("-q");
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let unbind_key = unbind_key.build().to_vec();

    assert_eq!(unbind_key, s);
}
