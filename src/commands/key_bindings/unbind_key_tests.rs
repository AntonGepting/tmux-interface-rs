// auto-generated file
//

// Unbind the command bound to key
//
// # Manual
//
// tmux >=3.2:
// ```text
// unbind-key [-anq] [-T key-table] key
// (alias: unbind)
// ```
//
// tmux >=2.4:
// ```text
// unbind-key [-an] [-T key-table] key
// (alias: unbind)
// ```
//
// tmux >=2.1:
// ```text
// unbind-key [-acn] [-t mode-table] [-T key-table] key
// (alias: unbind)
// ```
//
// tmux >=2.0:
// ```text
// unbind-key [-acn] [-t mode-table] key
// (alias: unbind)
// ```
//
// tmux >=1.5:
// ```text
// unbind-key [-acn] [-t key-table] key
// (alias: unbind)
// ```
//
// tmux >=0.8:
// ```text
// unbind-key key
// (alias: unbind)
// ```
#[test]
fn unbind_key() {
    use crate::UnbindKey;
    use std::borrow::Cow;

    let unbind_key = UnbindKey::new();
    // `[-a]`
    #[cfg(feature = "tmux_1_5")]
    let unbind_key = unbind_key.all();

    // `[-c]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    let unbind_key = unbind_key.command_mode();

    // `[-n]`
    #[cfg(feature = "tmux_1_5")]
    let unbind_key = unbind_key.root();

    // `[-q]`
    #[cfg(feature = "tmux_3_2")]
    let unbind_key = unbind_key.quiet();

    // `[-t key-table]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let unbind_key = unbind_key.key_table("1");

    // `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    let unbind_key = unbind_key.mode_table("2");

    // `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    let unbind_key = unbind_key.key_table("3");

    // `[key]`
    #[cfg(feature = "tmux_0_8")]
    let unbind_key = unbind_key.key("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unbind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unbind";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    v.push("-a");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_4")))]
    v.push("-c");
    #[cfg(feature = "tmux_1_5")]
    v.push("-n");
    #[cfg(feature = "tmux_3_2")]
    v.push("-q");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_1")]
    v.extend_from_slice(&["-T", "3"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("4");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let unbind_key = unbind_key.build().to_vec();

    assert_eq!(unbind_key, v);
}
