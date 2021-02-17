#[test]
fn list_windows() {
    use crate::{Error, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.6:
        // ```text
        // tmux list-windows [-a] [-F format] [-t target-session]
        // (alias: lsw)
        // ```
        //
        // tmux ^1.5:
        // ```text
        // tmux list-windows [-a] [-t target-session]
        // (alias: lsw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux list-windows [-t target-session]
        // (alias: lsw)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("list-windows");
        #[cfg(feature = "use_cmd_alias")]
        s.push("lsw");
        #[cfg(feature = "tmux_1_5")]
        s.push("-a");
        #[cfg(feature = "tmux_1_6")]
        s.extend_from_slice(&["-F", "1"]);
        #[cfg(feature = "tmux_0_8")]
        s.extend_from_slice(&["-t", "2"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_session = TargetSession::Raw("2").to_string();
    tmux.list_windows(Some(true), Some("1"), Some(&target_session))
        .unwrap_err();
}
