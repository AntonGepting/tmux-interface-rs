#[test]
fn show_hooks() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-hooks [-g] [-t target-session]
        // (alias: lockc)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-hooks", "-g", "-t", "1"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.show_hooks(Some(true), Some("1")).unwrap_err();
}
