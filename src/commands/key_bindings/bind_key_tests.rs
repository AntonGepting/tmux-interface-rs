#[test]
fn bind_key() {
    use crate::{BindKey, BindKeyBuilder, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
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
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("bind-key");
        #[cfg(feature = "use_cmd_alias")]
        s.push("bind");
        #[cfg(feature = "tmux_1_0")]
        s.push("-n");
        #[cfg(feature = "tmux_0_8")]
        s.push("-r");
        #[cfg(feature = "tmux_3_1")]
        s.extend_from_slice(&["-N", "1"]);
        #[cfg(feature = "tmux_2_1")]
        s.extend_from_slice(&["-T", "2"]);
        s.push("3");
        s.push("4");
        #[cfg(feature = "tmux_0_8")]
        s.push("5");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));

    let bind_key = BindKey {
        #[cfg(feature = "tmux_1_0")]
        root: Some(true),
        #[cfg(feature = "tmux_0_8")]
        repeat: Some(true),
        #[cfg(feature = "tmux_3_1")]
        note: Some("1"),
        #[cfg(feature = "tmux_2_1")]
        key_table: Some("2"),
        #[cfg(feature = "tmux_0_8")]
        arguments: Some("5"),
    };
    tmux.bind_key(Some(&bind_key), "3", "4").unwrap_err();

    let mut builder = BindKeyBuilder::new();
    #[cfg(feature = "tmux_1_0")]
    builder.root();
    #[cfg(feature = "tmux_0_8")]
    builder.repeat();
    #[cfg(feature = "tmux_3_1")]
    builder.note("1");
    #[cfg(feature = "tmux_2_1")]
    builder.key_table("2");
    #[cfg(feature = "tmux_0_8")]
    builder.arguments("5");
    let bind_key = builder.build();
    tmux.bind_key(Some(&bind_key), "3", "4").unwrap_err();
}
