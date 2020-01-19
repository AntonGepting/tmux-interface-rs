#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn set_option() {
    use crate::{Error, SetOption, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-option [-aFgopqsuw] [-t target-pane] option value
        // (alias: set)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-option", "-a", "-F", "-g", "-o", "-p", "-q", "-s", "-u", "-w", "-t", "1", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let set_option = SetOption {
        append: Some(true),
        format: Some(true),
        global: Some(true),
        not_overwrite: Some(true),
        pane: Some(true),
        quiet: Some(true),
        server: Some(true),
        unset: Some(true),
        window: Some(true),
        target: Some("1"),
    };
    tmux.set_option(Some(&set_option), "2", "3").unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn set_option() {
    use crate::{Error, SetOption, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux set-option [-aFgoqsuw] [-t target-session | target-window] option value
        // (alias: set)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["set-option", "-a", "-F", "-g", "-o", "-q", "-s", "-u", "-w", "-t", "1", "2", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let set_option = SetOption {
        append: Some(true),
        format: Some(true),
        global: Some(true),
        not_overwrite: Some(true),
        quiet: Some(true),
        server: Some(true),
        unset: Some(true),
        window: Some(true),
        target: Some("1"),
    };
    tmux.set_option(Some(&set_option), "2", "3").unwrap_err();
}
