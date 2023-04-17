#[test]
fn get_global_session_option_tests() {
    use crate::{GetGlobalSessionOption, GetSessionOptionTr, GetUserOption};

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd = "show";

    let target = ":";
    let cmd = format!("{} -g -t {}", cmd, target);

    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {}", cmd, "activity-action");
        let set_option = GetGlobalSessionOption::activity_action(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_8")]
    {
        let origin = format!("{} {}", cmd, "assume-paste-time");
        let set_option = GetGlobalSessionOption::assume_paste_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "base-index");
        let set_option = GetGlobalSessionOption::base_index(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "bell-action");
        let set_option = GetGlobalSessionOption::bell_action(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    //let options = options.bell_on_alert();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    //let options = options.buffer_limit();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "default-command");
        let set_option = GetGlobalSessionOption::default_command(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "default-shell");
        let set_option = GetGlobalSessionOption::default_shell(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.default_path();
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "default-size");
        let set_option = GetGlobalSessionOption::default_size(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    //let options = options.default_terminal();
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "destroy-unattached");
        let set_option = GetGlobalSessionOption::destroy_unattached(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "detach-on-destroy");
        let set_option = GetGlobalSessionOption::detach_on_destroy(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {}", cmd, "display-panes-active-colour");
        let set_option =
            GetGlobalSessionOption::display_panes_active_colour(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "display-panes-colour");
        let set_option = GetGlobalSessionOption::display_panes_colour(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "display-panes-time");
        let set_option = GetGlobalSessionOption::display_panes_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "display-time");
        let set_option = GetGlobalSessionOption::display_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "history-limit");
        let set_option = GetGlobalSessionOption::history_limit(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_2")]
    {
        let origin = format!("{} {}", cmd, "key-table");
        let set_option = GetGlobalSessionOption::key_table(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "lock-after-time");
        let set_option = GetGlobalSessionOption::lock_after_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_1")]
    {
        let origin = format!("{} {}", cmd, "lock-command");
        let set_option = GetGlobalSessionOption::lock_command(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    //let options = options.lock_server();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_attr();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_bg();
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_attr();
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_bg();
    //#[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    //let options = options.message_command_fg();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.message_fg();
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "message-command-style");
        let set_option = GetGlobalSessionOption::message_command_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //let options = options.message_limit();
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "message-style");
        let set_option = GetGlobalSessionOption::message_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_resize_pane();
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_pane();
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    //let options = options.mouse_select_window();
    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {}", cmd, "mouse");
        let set_option = GetGlobalSessionOption::mouse(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    //let options = options.mouse_utf8();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_bg();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_active_border_fg();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_bg();
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    //let options = options.pane_border_fg();
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_active_border_style();
    //#[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    //let options = options.pane_border_style();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "prefix");
        let set_option = GetGlobalSessionOption::prefix(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {}", cmd, "prefix2");
        let set_option = GetGlobalSessionOption::prefix2(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {}", cmd, "renumber-windows");
        let set_option = GetGlobalSessionOption::renumber_windows(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "repeat-time");
        let set_option = GetGlobalSessionOption::repeat_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let options = options.set_remain_on_exit();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "set-titles");
        let set_option = GetGlobalSessionOption::set_titles(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "set-titles-string");
        let set_option = GetGlobalSessionOption::set_titles_string(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_6")]
    {
        let origin = format!("{} {}", cmd, "silence-action");
        let set_option = GetGlobalSessionOption::silence_action(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status");
        let set_option = GetGlobalSessionOption::status(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "status-bg");
        let set_option = GetGlobalSessionOption::status_bg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    {
        let origin = format!("{} {}", cmd, "status-fg");
        let set_option = GetGlobalSessionOption::status_fg(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "assume-paste-time");
        let set_option = GetGlobalSessionOption::assume_paste_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_2_9")]
    {
        let origin = format!("{} {}", cmd, "status-format");
        let set_option = GetGlobalSessionOption::status_format(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-interval");
        let set_option = GetGlobalSessionOption::status_interval(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-justify");
        let set_option = GetGlobalSessionOption::status_justify(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-keys");
        let set_option = GetGlobalSessionOption::status_keys(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-left");
        let set_option = GetGlobalSessionOption::status_left(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_attr();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_bg();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_left_fg();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-left-length");
        let set_option = GetGlobalSessionOption::status_left_length(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "status-left-style");
        let set_option = GetGlobalSessionOption::status_left_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_7")]
    {
        let origin = format!("{} {}", cmd, "status-position");
        let set_option = GetGlobalSessionOption::status_position(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-right");
        let set_option = GetGlobalSessionOption::status_right(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_attr();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_bg();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    //let options = options.status_right_fg();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "status-right-length");
        let set_option = GetGlobalSessionOption::status_right_length(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "status-right-style");
        let set_option = GetGlobalSessionOption::status_right_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "status-style");
        let set_option = GetGlobalSessionOption::status_style(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    //let options = options.status_utf8();
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //let options = options.terminal_overrides();
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "assume-paste-time");
        let set_option = GetGlobalSessionOption::assume_paste_time(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "update-environment");
        let set_option = GetGlobalSessionOption::update_environment(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "visual-activity");
        let set_option = GetGlobalSessionOption::visual_activity(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_0")]
    {
        let origin = format!("{} {}", cmd, "visual-bell");
        let set_option = GetGlobalSessionOption::visual_bell(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    //v.push(format!("{} {}", cmd, "visual-content", "fg=colour247"));
    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "visual-silence");
        let set_option = GetGlobalSessionOption::visual_silence(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    #[cfg(feature = "tmux_1_6")]
    {
        let origin = format!("{} {}", cmd, "word-separators");
        let set_option = GetGlobalSessionOption::word_separators(Some(target)).to_string();
        assert_eq!(origin, set_option);
    }
    {
        let origin = format!("{} {}", cmd, "@user-option-name");
        let set_option =
            GetGlobalSessionOption::user_option_ext(Some(target), "user-option-name").to_string();
        assert_eq!(origin, set_option);
    }
}
