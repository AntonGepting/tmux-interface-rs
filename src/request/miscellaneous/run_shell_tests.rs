#[test]
fn run_shell() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux run-shell [-b] [-t target-pane] shell-command
        // (alias: run)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["run-shell", "-b", "-t", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.run_shell(Some(true), Some(&TargetPane::Raw("1")), "2")
        .unwrap_err();
}
