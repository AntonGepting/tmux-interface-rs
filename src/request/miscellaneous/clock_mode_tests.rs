#[test]
fn clock_mode() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux clock-mode [-t target-pane]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["clock-mode", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    let target_pane = TargetPane::Raw("1").to_string();
    tmux.clock_mode(Some(&target_pane)).unwrap_err();
}
