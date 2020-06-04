#[test]
fn copy_mode() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux copy-mode [-Meu] [-t target-pane]
        // ```
        //
        // tmux ^1.0:
        // ```text
        // tmux copy-mode [-u] [-t target-pane]
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux copy-mode [-u] [-t target-window]
        // ```
        assert_eq!(
            format!("{:?} {:?} {:?}", bin, options, subcmd),
            r#""tmux" [] ["copy-mode", "-M", "-e", "-u", "-t", "1"]"#
        );
        let mut s = Vec::new();
        let o: Vec<&str> = Vec::new();
        s.push("copy-mode");
        s.push("-M");
        s.push("-e");
        s.push("-u");
        s.extend_from_slice(&["-t", "1"]);
        assert_eq!(bin, "tmux");
        assert_eq!(options, &o);
        assert_eq!(subcmd, &s);
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    tmux.copy_mode(Some(true), Some(true), Some(true), Some(&target_pane))
        .unwrap_err();
}
