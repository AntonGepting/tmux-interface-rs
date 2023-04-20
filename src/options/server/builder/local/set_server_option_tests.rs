#[test]
fn set_server_option() {
    use crate::{SetClipboard, SetServerOption, SetServerOptionTr, SetUserOption, Switch};

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let cmd = format!("{} -s", cmd);

    #[cfg(feature = "tmux_3_1")]
    {
        let origin = format!("{} {} {}", cmd, "backspace", "C-?");
        let set_option = SetServerOption::backspace(Some("C-?")).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_1_5")]
    {
        let origin = format!("{} {} {}", cmd, "buffer-limit", "50");
        let set_option = SetServerOption::buffer_limit(Some(50)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_2_4")]
    {
        let origin0 = format!(
            "{} {} {}",
            cmd, "command-alias[0]", "split-pane=split-window"
        );
        let origin1 = format!("{} {} {}", cmd, "command-alias[1]", "splitp=split-window");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetServerOption::command_alias(Some(vec![
            "split-pane=split-window".to_string(),
            "splitp=split-window".to_string(),
        ]))
        .to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {} {}", cmd, "copy-command", "");
        let set_option = SetServerOption::copy_command(Some("")).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {} {}", cmd, "default-terminal", "screen-256color");
        let set_option = SetServerOption::default_terminal(Some("screen-256color")).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {} {}", cmd, "escape-time", "500");
        let set_option = SetServerOption::escape_time(Some(500)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {} {}", cmd, "editor", "/usr/bin/vi");
        let set_option = SetServerOption::editor(Some("/usr/bin/vi")).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_2_7")]
    {
        let origin = format!("{} {} {}", cmd, "exit-empty", "on");
        let set_option = SetServerOption::exit_empty(Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "exit-unattached", "off");
        let set_option = SetServerOption::exit_unattached(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {} {}", cmd, "extended-keys", "off");
        let set_option = SetServerOption::extended_keys(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "focus-events", "off");
        let set_option = SetServerOption::focus_events(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {} {}", cmd, "history-file", "");
        let set_option = SetServerOption::history_file(Some("")).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_2_0")]
    {
        let origin = format!("{} {} {}", cmd, "message-limit", "1000");
        let set_option = SetServerOption::message_limit(Some(1000)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_3_3")]
    {
        let origin = format!("{} {} {}", cmd, "prompt-history-limit", "100");
        let set_option = SetServerOption::prompt_history_limit(Some(100)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_1_5")]
    {
        let origin = format!("{} {} {}", cmd, "set-clipboard", "off");
        let set_option = SetServerOption::set_clipboard(Some(SetClipboard::Off)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin0 = format!(
            "{} {} {}",
            cmd, "terminal-features[0]", "xterm*:clipboard:ccolour:cstyle:focus:title"
        );
        let origin1 = format!("{} {} {}", cmd, "terminal-features[1]", "screen*:title");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetServerOption::terminal_features(Some(vec![
            "xterm*:clipboard:ccolour:cstyle:focus:title".to_string(),
            "screen*:title".to_string(),
        ]))
        .to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_2_0")]
    {
        let origin = format!("{} {} {}", cmd, "terminal-overrides", "");
        let set_option = SetServerOption::terminal_overrides(None::<Vec<String>>).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {} {}", cmd, "user-keys", "");
        let set_option = SetServerOption::user_keys(None::<Vec<String>>).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    {
        let origin = format!("{} {} {}", cmd, "quiet", "off");
        let set_option = SetServerOption::quiet(Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }

    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    {
        let origin = format!("{} {} {}", cmd, "detach-on-destroy", "");
        let set_option = SetServerOption::detach_on_destroy(Some("")).to_string();
        assert_eq!(origin, set_option);
    }

    {
        let origin = format!("{} {} {}", cmd, "@user-option-name", "value");
        let set_option =
            SetServerOption::user_option("user-option-name", Some("value".to_string())).to_string();
        assert_eq!(origin, set_option);
    }
}
