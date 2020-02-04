#[test]
fn next_layout() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux next-layout [-t target-window]
        // (alias: nextl)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["next-layout", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.next_layout(Some("1")).unwrap_err();
}