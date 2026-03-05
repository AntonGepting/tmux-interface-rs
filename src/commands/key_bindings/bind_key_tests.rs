// auto-generated file
//

// Bind key `key` to command
//
// # Manual
//
// tmux >=3.1:
// ```text
// bind-key [-nr] [-N note] [-T key-table] key command [arguments]
// (alias: bind)
// ```
//
// tmux >=2.4:
// ```text
// bind-key [-nr] [-T key-table] key command [arguments]
// (alias: bind)
// ```
//
// tmux >=2.3:
// ```text
// bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
// (alias: bind)
// ```
//
// tmux >=2.1:
// ```text
// bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
// (alias: bind)
// ```
//
// tmux >=2.0:
// ```text
// bind-key [-cnr] [-t mode-table] key command [arguments]
// (alias: bind)
// ```
//
// tmux >=1.5:
// ```text
// bind-key [-cnr] [-t key-table] key command [arguments]
// (alias: bind)
// ```
//
// tmux >=0.8:
// ```text
// bind-key [-r] key command [arguments]
// (alias: bind)
// ```
#[test]
fn bind_key() {
    use crate::BindKey;
    use std::borrow::Cow;

    let bind_key = BindKey::new();
    // `[-c]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    let bind_key = bind_key.command_mode();

    // `[-n]`
    #[cfg(feature = "tmux_1_5")]
    let bind_key = bind_key.root();

    // `[-r]`
    #[cfg(feature = "tmux_1_5")]
    let bind_key = bind_key.repeat();

    // `[-N note]`
    #[cfg(feature = "tmux_3_1")]
    let bind_key = bind_key.note("1");

    // `[-R repeat-count]`
    #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
    let bind_key = bind_key.repeat_count("2");

    // `[-t key-table]`
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let bind_key = bind_key.key_table("3");

    // `[-t mode-table]`
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    let bind_key = bind_key.mode_table("4");

    // `[-T key-table]`
    #[cfg(feature = "tmux_2_1")]
    let bind_key = bind_key.key_table("5");

    // `[key]`
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.key("6");

    // `[command]`
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.command("7");

    // `[arguments]`
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.arguments("8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "bind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "bind";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    v.push("-c");
    #[cfg(feature = "tmux_1_5")]
    v.push("-n");
    #[cfg(feature = "tmux_1_5")]
    v.push("-r");
    #[cfg(feature = "tmux_3_1")]
    v.extend_from_slice(&["-N", "1"]);
    #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
    v.extend_from_slice(&["-R", "2"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    v.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    v.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_1")]
    v.extend_from_slice(&["-T", "5"]);
    #[cfg(feature = "tmux_0_8")]
    v.push("6");
    #[cfg(feature = "tmux_0_8")]
    v.push("7");
    #[cfg(feature = "tmux_0_8")]
    v.push("8");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let bind_key = bind_key.build().to_vec();

    assert_eq!(bind_key, v);
}
