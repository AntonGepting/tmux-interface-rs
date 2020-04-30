#[test]
fn confirm_before() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.5:
        // ```text
        // tmux confirm-before [-p prompt] [-t target-client] command
        // (alias: confirm)
        // ```
        //
        // tmux ^0.9:
        // ```text
        // tmux confirm-before [-t target-client] command
        // (alias: confirm)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["confirm-before", "-p", "1", "-t", "2", "3"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("confirm_before");
        #[cfg(feature = "tmux_1_5")]
        s.extend_from_slice(&["-p", "1"]);
        #[cfg(feature = "tmux_0_9")]
        s.extend_from_slice(&["-t", "2"]);
        s.push("3");
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    tmux.confirm_before(Some("1"), Some("2"), "3").unwrap_err();
}
