#[test]
fn bind_key() {
    use crate::BindKey;
    use std::borrow::Cow;

    // Structure binding key `key` to command
    //
    // # Manual
    //
    // tmux 3.1:
    // ```text
    // tmux bind-key [-nr] [-N note] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // tmux bind-key [-nr] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // tmux bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // tmux bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // tmux bind-key [-cnr] [-t mode-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // tmux bind-key [-cnr] [-t key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux bind-key [-r] key command [arguments]
    // (alias: bind)
    // ```

    let bind_key = BindKey::new();
    #[cfg(feature = "tmux_1_0")]
    let bind_key = bind_key.root();
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.repeat();
    #[cfg(feature = "tmux_3_1")]
    let bind_key = bind_key.note("1");
    #[cfg(feature = "tmux_2_1")]
    let bind_key = bind_key.key_table("2");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.arguments("3");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.key("4");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key.command("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "bind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "bind";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.push("-n");
    #[cfg(feature = "tmux_0_8")]
    s.push("-r");
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-N", "1"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let bind_key = bind_key.build().to_vec();

    assert_eq!(bind_key, s);
}
