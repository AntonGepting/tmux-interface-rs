#[test]
fn get_session_options() {
    use crate::options::session::get_session_options::GetGlobalSessionOptions;
    use crate::options::session::get_session_options::GetSessionOptions;

    let options = GetGlobalSessionOptions::new();
    #[cfg(feature = "tmux_2_6")]
    let options = options.activity_action();
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time();
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index();
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action();
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit();
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command();
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal();
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size();
    #[cfg(feature = "tmux_1_4")]
    let options = options.destroy_unattached();
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy();
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour();
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour();
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time();
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time();
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit();
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table();
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time();
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command();
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg();
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr();
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg();
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg();
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style();
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit();
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style();
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane();
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane();
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window();
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse();
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8();
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg();
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg();
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg();
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg();
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style();
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style();
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix();
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2();
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows();
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit();
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles();
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles_string();
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg();
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length();
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style();
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg();
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length();
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style();
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides();
    #[cfg(feature = "tmux_1_0")]
    let options = options.update_environment();
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    let options = options.user_keys();
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity();
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content();
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence();
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators();

    let options = options.options.to_string();

    let cmd = "show -g";
    let separator = " ; ";

    let mut origin = Vec::new();
    #[cfg(feature = "tmux_2_6")]
    origin.push(format!("{} {}", cmd, "activity-action"));
    #[cfg(feature = "tmux_1_8")]
    origin.push(format!("{} {}", cmd, "assume-paste-time"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "base-index"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "bell-action"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    origin.push(format!("{} {}", cmd, "bell-on-alert"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    origin.push(format!("{} {}", cmd, "buffer-limit"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "default-command"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "default-shell"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "default-path"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "default-terminal"));
    #[cfg(feature = "tmux_2_9")]
    origin.push(format!("{} {}", cmd, "default-size"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "destroy-unattached"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "detach-on-destroy"));
    #[cfg(feature = "tmux_1_2")]
    origin.push(format!("{} {}", cmd, "display-panes-active-colour"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "display-panes-colour"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "display-panes-time"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "display-time"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "history-limit"));
    #[cfg(feature = "tmux_2_2")]
    origin.push(format!("{} {}", cmd, "key-table"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "lock-after-time"));
    #[cfg(feature = "tmux_1_1")]
    origin.push(format!("{} {}", cmd, "lock-command"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "lock-server"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-command-attr"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-command-bg"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-command-fg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "message-fg"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "message-command-style"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "message-limit"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "message-style"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mouse-resize-pane"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mouse-select-pane"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {}", cmd, "mouse-select-window"));
    #[cfg(feature = "tmux_2_1")]
    origin.push(format!("{} {}", cmd, "mouse"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    origin.push(format!("{} {}", cmd, "mouse-utf8"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-active-border-bg"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-active-border-fg"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-border-bg"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "pane-border-fg"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "pane-active-border-style"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "pane-border-style"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "prefix"));
    #[cfg(feature = "tmux_1_6")]
    origin.push(format!("{} {}", cmd, "prefix2"));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {}", cmd, "renumber-windows"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "repeat-time"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    origin.push(format!("{} {}", cmd, "set-remain-on-exit"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "set-titles"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "set-titles-string"));
    #[cfg(feature = "tmux_2_6")]
    origin.push(format!("{} {}", cmd, "silence-action"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-fg"));
    #[cfg(feature = "tmux_2_9")]
    origin.push(format!("{} {}", cmd, "status-format"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-interval"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-justify"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-keys"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-left"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-left-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-left-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-left-fg"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-left-length"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "status-left-style"));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {}", cmd, "status-position"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-right"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-right-attr"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-right-bg"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {}", cmd, "status-right-fg"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "status-right-length"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "status-right-style"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {}", cmd, "status-style"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    origin.push(format!("{} {}", cmd, "status-utf8"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "terminal-overrides"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "update-environment"));
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {}", cmd, "user-keys"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "visual-activity"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {}", cmd, "visual-bell"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {}", cmd, "visual-content"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {}", cmd, "visual-silence"));
    #[cfg(feature = "tmux_1_6")]
    origin.push(format!("{} {}", cmd, "word-separators"));

    let origin = origin.join(separator);

    assert_eq!(options, origin);
}

//#[test]
//fn get_session_options() {
//use crate::Tmux;

//let get_options = GetGlobalSessionOptions::new()
//.visual_silence()
//.word_separators()
//.into_commands();

//let output = Tmux::new().commands(get_options).output();

//dbg!(output);

//let get_options = GetLocalSessionOptions::new()
//.visual_silence()
//.word_separators()
//.into_commands();

//let output = Tmux::new().commands(get_options).output();

//dbg!(output);
//}
