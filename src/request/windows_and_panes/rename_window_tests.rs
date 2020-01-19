#[test]
fn rename_window() {
    use crate::{Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux rename-window [-t target-window] new-name
        // (alias: renamew)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["rename-window", "-t", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    tmux.rename_window(Some("1"), "2").unwrap_err();
}
