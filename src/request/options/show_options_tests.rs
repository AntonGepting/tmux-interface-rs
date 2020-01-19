#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn show_options() {
    use crate::{Error, ShowOptions, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-options [-AgHpqsvw] [-t target-pane] [option]
        // (alias: show)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-options", "-A", "-g", "-H", "-p", "-q", "-s", "-v", "-w", "-t", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    let show_options = ShowOptions {
        include_inherited: Some(true),
        global_options: Some(true),
        hooks: Some(true),
        pane: Some(true),
        quiet: Some(true),
        server: Some(true),
        option_value: Some(true),
        window: Some(true),
        target: Some("1"),
        option: Some("2"),
    };
    tmux.show_options(Some(&show_options)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn show_options() {
    use crate::{Error, ShowOptions, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux show-options [-gqsvw] [-t target-session | target-window] [option]
        // (alias: show)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["show-options", "-g", "-q", "-s", "-v", "-w", "-t", "1", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    let show_options = ShowOptions {
        global_options: Some(true),
        quiet: Some(true),
        server: Some(true),
        option_value: Some(true),
        window: Some(true),
        target: Some("1"),
        option: Some("2"),
    };
    tmux.show_options(Some(&show_options)).unwrap_err();
}
