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

    let mut bind_key = BindKey::new();
    #[cfg(feature = "tmux_1_0")]
    bind_key.root();
    #[cfg(feature = "tmux_0_8")]
    bind_key.repeat();
    #[cfg(feature = "tmux_3_1")]
    bind_key.note("1");
    #[cfg(feature = "tmux_2_1")]
    bind_key.key_table("2");
    #[cfg(feature = "tmux_0_8")]
    bind_key.arguments("3");
    #[cfg(feature = "tmux_0_8")]
    bind_key.key("4");
    #[cfg(feature = "tmux_0_8")]
    bind_key.command("5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "bind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "bind";

    let mut s = Vec::new();
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
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(bind_key.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(bind_key.0.bin_args, None);
    assert_eq!(bind_key.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(bind_key.0.cmd_args, Some(s));
}
