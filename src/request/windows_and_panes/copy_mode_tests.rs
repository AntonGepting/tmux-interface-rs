#[test]
fn copy_mode() {
    use crate::{Error, TargetPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux copy-mode [-Meu] [-t target-pane]
        assert_eq!(
            format!("{:?} {:?} {:?}", bin, options, subcmd),
            r#""tmux" [] ["copy-mode", "-M", "-e", "-u", "-t", "1"]"#
        );
        Err(Error::Hook)
    }));
    tmux.copy_mode(
        Some(true),
        Some(true),
        Some(true),
        Some(&TargetPane::Raw("1")),
    )
    .unwrap_err();
}
