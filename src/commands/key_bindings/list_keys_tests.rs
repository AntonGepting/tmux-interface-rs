// auto-generated file
//

// List key bindings
//
// # Manual
//
// tmux >=3.1a:
// ```text
// list-keys [-1aN] [-P prefix-string -T key-table]
// (alias: lsk)
// ```
//
// tmux >=3.1:
// ```text
// list-keys [-1N] [-P prefix-string -T key-table] [key]
// (alias: lsk)
// ```
//
// tmux >=2.4:
// ```text
// list-keys [-T key-table]
// (alias: lsk)
// ```
//
// tmux >=2.1:
// ```text
// list-keys [-t mode-table] [-T key-table]
// (alias: lsk)
// ```
//
// tmux >=0.8:
// ```text
// list-keys [-t key-table]
// (alias: lsk)
// ```
#[test]
fn list_keys() {
    use crate::ListKeys;
    use std::borrow::Cow;

    let list_keys = ListKeys::new();
    // `[-1]`
    #[cfg(feature = "tmux_3_1")]
    let list_keys = list_keys.first();

    // `[-a]`
    #[cfg(feature = "tmux_3_1a")]
    let list_keys = list_keys.command();

    // `[-N]`
    #[cfg(feature = "tmux_3_1")]
    let list_keys = list_keys.with_notes();

    // `[-P prefix-string]`
    #[cfg(feature = "tmux_3_1")]
    let list_keys = list_keys.prefix_string("1");

    // `[-t key-table]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    let list_keys = list_keys.key_table("2");

    // `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    let list_keys = list_keys.mode_table("3");

    // `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    let list_keys = list_keys.key_table("4");

    // `[key]`
    #[cfg(feature = "tmux_3_1")]
    let list_keys = list_keys.key("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsk";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_3_1")]
    v.push("-1");
    #[cfg(feature = "tmux_3_1a")]
    v.push("-a");
    #[cfg(feature = "tmux_3_1")]
    v.push("-N");
    #[cfg(feature = "tmux_3_1")]
    v.extend_from_slice(&["-P", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    v.extend_from_slice(&["-t", "2"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_2_1")]
    v.extend_from_slice(&["-T", "4"]);
    #[cfg(feature = "tmux_3_1")]
    v.push("5");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let list_keys = list_keys.build().to_vec();

    assert_eq!(list_keys, v);
}
