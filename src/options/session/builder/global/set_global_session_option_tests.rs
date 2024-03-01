#[test]
fn set_global_session_option_tests() {
    use crate::{
        Action, Activity, DestroyUnattached, DetachOnDestroy, SetGlobalSessionOption,
        SetSessionOptionTr, SetUserOption, Status, StatusJustify, StatusKeys, StatusPosition,
        Switch,
    };

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let target = ":";
    let cmd = format!("{} -g -t {}", cmd, target);

    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {} {}", cmd, "activity-action", "other");
        let set_option =
            SetGlobalSessionOption::activity_action(Some(target), Some(Action::Other)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_8")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option =
            SetGlobalSessionOption::assume_paste_time(Some(target), Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "base-index", "1");
        let set_option = SetGlobalSessionOption::base_index(Some(target), Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "bell-action", "none");
        let set_option =
            SetGlobalSessionOption::bell_action(Some(target), Some(Action::None)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let options = options.bell_on_alert(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit(Some(target), Some(3));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "default-command", "");
        let set_option =
            SetGlobalSessionOption::default_command(Some(target), Some("")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "default-shell", "/bin/bash");
        let set_option =
            SetGlobalSessionOption::default_shell(Some(target), Some("/bin/bash")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path(Some(target), Some("6"));
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {} {}", cmd, "default-size", "80x24");
        let set_option =
            SetGlobalSessionOption::default_size(Some(target), Some((80, 24))).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal(Some(target), Some("8"));
    #[cfg(feature = "tmux_1_5")]
    {
        let origin = format!("{} {} {}", cmd, "destroy-unattached", "off");
        let set_option =
            SetGlobalSessionOption::destroy_unattached(Some(target), Some(DestroyUnattached::Off))
                .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "detach-on-destroy", "on");
        let set_option =
            SetGlobalSessionOption::detach_on_destroy(Some(target), Some(DetachOnDestroy::On))
                .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-active-colour", "red");
        let set_option =
            SetGlobalSessionOption::display_panes_active_colour(Some(target), Some("red"))
                .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-colour", "blue");
        let set_option =
            SetGlobalSessionOption::display_panes_colour(Some(target), Some("blue")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-panes-time", "1000");
        let set_option =
            SetGlobalSessionOption::display_panes_time(Some(target), Some(1000)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "display-time", "750");
        let set_option = SetGlobalSessionOption::display_time(Some(target), Some(750)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "history-limit", "2000");
        let set_option =
            SetGlobalSessionOption::history_limit(Some(target), Some(2000)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_2")]
    {
        let origin = format!("{} {} {}", cmd, "key-table", "root");
        let set_option = SetGlobalSessionOption::key_table(Some(target), Some("root")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "lock-after-time", "0");
        let set_option = SetGlobalSessionOption::lock_after_time(Some(target), Some(0)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_1")]
    {
        let origin = format!("{} {} {}", cmd, "lock-command", "lock -np");
        let set_option =
            SetGlobalSessionOption::lock_command(Some(target), Some("lock -np")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    //let options = options.lock_server(Some(target), Some("17"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_attr(Some(target), Some("18"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_bg(Some(target), Some("19"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_attr(Some(target), Some("20"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_bg(Some(target), Some("21"));
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_fg(Some(target), Some("22"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_fg(Some(target), Some("23"));
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "message-command-style", "fg=blue,bg=black");
        let set_option =
            SetGlobalSessionOption::message_command_style(Some(target), Some("fg=blue,bg=black"))
                .to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit(Some(target), Some(25));
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!(
            "{} {} {}",
            cmd, "message-style", "fg=colour232,bg=colour166,bold"
        );
        let set_option = SetGlobalSessionOption::message_style(
            Some(target),
            Some("fg=colour232,bg=colour166,bold"),
        )
        .to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {} {}", cmd, "mouse", "on");
        let set_option = SetGlobalSessionOption::mouse(Some(target), Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let options = options.mouse_utf8(Some(target), Some(Switch::On));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_bg(Some(target), Some("27"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_fg(Some(target), Some("28"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_bg(Some(target), Some("29"));
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_fg(Some(target), Some("30"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_active_border_style(Some(target), Some("31"));
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_border_style(Some(target), Some("32"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "prefix", "C-a");
        let set_option = SetGlobalSessionOption::prefix(Some(target), Some("C-a")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {} {}", cmd, "prefix2", "None");
        let set_option = SetGlobalSessionOption::prefix2(Some(target), Some("None")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {} {}", cmd, "renumber-windows", "off");
        let set_option =
            SetGlobalSessionOption::renumber_windows(Some(target), Some(Switch::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "repeat-time", "500");
        let set_option = SetGlobalSessionOption::repeat_time(Some(target), Some(500)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "set-titles", "on");
        let set_option =
            SetGlobalSessionOption::set_titles(Some(target), Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "set-titles-string", "#W");
        let set_option =
            SetGlobalSessionOption::set_titles_string(Some(target), Some("#W")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {} {}", cmd, "silence-action", "other");
        let set_option =
            SetGlobalSessionOption::silence_action(Some(target), Some(Action::Other)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status", "on");
        let set_option = SetGlobalSessionOption::status(Some(target), Some(Status::On)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr(Some(target), Some("36"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {} {}", cmd, "status-bg", "default");
        let set_option =
            SetGlobalSessionOption::status_bg(Some(target), Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {} {}", cmd, "status-fg", "colour12");
        let set_option =
            SetGlobalSessionOption::status_fg(Some(target), Some("colour12")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {} {}", cmd, "assume-paste-time", "1");
        let set_option =
            SetGlobalSessionOption::assume_paste_time(Some(target), Some(1)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin0 = format!("{} {} {}", cmd, "status-format[0]", "#[0]");
        let origin1 = format!("{} {} {}", cmd, "status-format[1]", "#[1]");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetGlobalSessionOption::status_format(
            Some(target),
            Some(vec!["#[0]".to_string(), "#[1]".to_string()]),
        )
        .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-interval", "2");
        let set_option = SetGlobalSessionOption::status_interval(Some(target), Some(2)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-justify", "left");
        let set_option =
            SetGlobalSessionOption::status_justify(Some(target), Some(StatusJustify::Left))
                .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-keys", "emacs");
        let set_option =
            SetGlobalSessionOption::status_keys(Some(target), Some(StatusKeys::Emacs)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-left", "#(whoami)");
        let set_option =
            SetGlobalSessionOption::status_left(Some(target), Some("#(whoami)")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_attr(Some(target), Some("41"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg(Some(target), Some("42"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg(Some(target), Some("43"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-left-length", "50");
        let set_option =
            SetGlobalSessionOption::status_left_length(Some(target), Some(50)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-left-style", "default");
        let set_option =
            SetGlobalSessionOption::status_left_style(Some(target), Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {} {}", cmd, "status-position", "bottom");
        let set_option =
            SetGlobalSessionOption::status_position(Some(target), Some(StatusPosition::Bottom))
                .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-right", "#[]");
        let set_option =
            SetGlobalSessionOption::status_right(Some(target), Some("#[]")).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr(Some(target), Some("47"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg(Some(target), Some("48"));
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg(Some(target), Some("49"));
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "status-right-length", "50");
        let set_option =
            SetGlobalSessionOption::status_right_length(Some(target), Some(50)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-right-style", "default");
        let set_option =
            SetGlobalSessionOption::status_right_style(Some(target), Some("default")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {} {}", cmd, "status-style", "fg=colour247");
        let set_option =
            SetGlobalSessionOption::status_style(Some(target), Some("fg=colour247")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    {
        let origin = format!("{} {} {}", cmd, "status-utf8", "on");
        let set_option =
            SetGlobalSessionOption::status_utf8(Some(target), Some(Switch::On)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    {
        let origin = format!("{} {} {}", cmd, "terminal-overrides", "1");
        let set_option =
            SetGlobalSessionOption::terminal_overrides(Some(target), Some("1")).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin0 = format!("{} {} {}", cmd, "update-environment[0]", "DISPLAY");
        let origin1 = format!("{} {} {}", cmd, "update-environment[1]", "KRB5CCNAME");
        let origin = format!("{} ; {}", origin0, origin1);
        let set_option = SetGlobalSessionOption::update_environment(
            Some(target),
            Some(vec!["DISPLAY".to_string(), "KRB5CCNAME".to_string()]),
        )
        .to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "visual-activity", "off");
        let set_option =
            SetGlobalSessionOption::visual_activity(Some(target), Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {} {}", cmd, "visual-bell", "off");
        let set_option =
            SetGlobalSessionOption::visual_bell(Some(target), Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //v.push(format!("{} {} {}", cmd, "visual-content", "fg=colour247"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {} {}", cmd, "visual-silence", "off");
        let set_option =
            SetGlobalSessionOption::visual_silence(Some(target), Some(Activity::Off)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!(
            "{} {} {}",
            cmd, "word-separators", "!\"#$%&'()*+,-./;<=>?@[\\]^`{|}~"
        );
        let set_option = SetGlobalSessionOption::word_separators(
            Some(target),
            Some("!\"#$%&'()*+,-./;<=>?@[\\]^`{|}~"),
        )
        .to_string();
        assert_eq!(origin, set_option);
    }
    {
        let origin = format!("{} {} {}", cmd, "@user-option-name", "value");
        let set_option = SetGlobalSessionOption::user_option_ext(
            Some(target),
            "user-option-name",
            Some("value"),
        )
        .to_string();
        assert_eq!(origin, set_option);
    }
}
