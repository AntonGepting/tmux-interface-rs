#[test]
fn set_window_option() {
    use crate::{Error, SetWindowOption, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-window-option [-aFgoqu] [-t target-window] option value
        // (alias: setw)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-window-option", "-a", "-F", "-g", "-o", "-q", "-u", "-t", "1", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let set_window_option = SetWindowOption {
        append: Some(true),
        format: Some(true),
        global: Some(true),
        not_overwrite: Some(true),
        quiet: Some(true),
        unset: Some(true),
        target_window: Some("1"),
    };
    tmux.set_window_option(Some(&set_window_option), "2", "3")
        .unwrap_err();
}
