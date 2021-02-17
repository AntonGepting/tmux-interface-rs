#[test]
fn previous_layout() {
    use crate::{Error, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^1.3:
        // ```text
        // tmux previous-layout [-t target-window]
        // (alias: prevl)
        // ```
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        #[cfg(not(feature = "use_cmd_alias"))]
        s.push("previous-layout");
        #[cfg(feature = "use_cmd_alias")]
        s.push("prevl");
        #[cfg(feature = "tmux_1_3")]
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_window = TargetWindow::Raw("1").to_string();
    tmux.previous_layout(Some(&target_window)).unwrap_err();
}
