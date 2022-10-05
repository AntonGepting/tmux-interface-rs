#[test]
fn set_window_options() {
    #[cfg(feature = "tmux_2_9")]
    use crate::WindowSize;
    use crate::{
        ClockModeStyle, PaneBorderStatus, SetGlobalWindowOptions, SetUserOptions, SetWindowOptions,
        StatusKeys, Switch,
    };

    let options = SetGlobalWindowOptions::new();

    //cursor-colour none
    //cursor-style default
    #[cfg(feature = "tmux_1_0")]
    let options = options.aggressive_resize(Some(Switch::Off));
    //allow-passthrough off
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    let options = options.allow_rename(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    let options = options.alternate_screen(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")] // 0.8
    let options = options.automatic_rename(Some(Switch::On));
    #[cfg(feature = "tmux_1_9")]
    let options = options.automatic_rename_format(Some(
        "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}".to_string(),
    ));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let options = options.c0_change_interval();
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let options = options.c0_change_trigger();
    #[cfg(feature = "tmux_1_0")]
    let options = options.clock_mode_colour(Some("colour135"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.clock_mode_style(Some(ClockModeStyle::_24));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let options = options.force_height(Some(0));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let options = options.force_width(Some(0));
    //copy-mode-match-style bg=cyan,fg=black
    //copy-mode-current-match-style bg=magenta,fg=black
    //copy-mode-mark-style bg=red,fg=black
    //fill-character ''

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    let options = options.layout_history_limit(Some(0));
    #[cfg(feature = "tmux_1_0")]
    let options = options.main_pane_height(Some(24));
    #[cfg(feature = "tmux_1_0")]
    let options = options.main_pane_width(Some(80));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_attr();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_bg();
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_fg();
    #[cfg(feature = "tmux_1_0")]
    let options = options.mode_keys(Some(StatusKeys::Vi));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.mode_mouse();
    #[cfg(feature = "tmux_1_9")]
    let options = options.mode_style(Some("fg=colour196,bg=colour238,bold"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.monitor_activity(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.monitor_content();
    #[cfg(feature = "tmux_2_6")]
    let options = options.monitor_bell(Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let options = options.monitor_silence(Some(0));

    #[cfg(feature = "tmux_1_4")]
    let options = options.other_pane_height(Some(0));
    #[cfg(feature = "tmux_1_4")]
    let options = options.other_pane_width(Some(0));
    #[cfg(feature = "tmux_2_0")]
    let options = options.pane_active_border_style(Some("fg=colour114,bg=colour235"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.pane_base_index(Some(0));
    #[cfg(feature = "tmux_2_3")]
    let options = options.pane_border_format(Some(
        "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    ));
    #[cfg(feature = "tmux_2_3")]
    let options = options.pane_border_status(Some(PaneBorderStatus::Off));
    //pane-border-indicators colour
    //pane-border-lines single
    //pane-colours
    #[cfg(feature = "tmux_2_0")]
    let options = options.pane_border_style(Some("fg=colour238,bg=colour235"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    let options = options.remain_on_exit(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    let options = options.synchronize_panes(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.utf8(Some(""));
    //popup-style default
    //popup-border-style default
    //popup-border-lines single
    //remain-on-exit-format "Pane is dead (#{?#{!=:#{pane_dead_status},},status #{pane_dead_status},}#{?#{!=:#{pane_dead_signal},},signal #{pane_dead_signal},}, #{t:pane_dead_time})"
    //scroll-on-clear on
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let options = options.window_active_style(Some("fg=colour253,bg=colour235"));
    //window-status-activity-style reverse
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_attr(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_bg(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_fg(Some(""));
    //window-status-bell-style fg=colour253,bg=colour1,bold
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_attr(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_bg(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_fg(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_attr(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_bg(Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_fg(Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_attr(Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_bg(Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_fg(Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_attr(Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_bg(Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_fg(Some(""));
    //window-status-current-format " #I: #W #F "
    //window-status-current-style fg=colour22,bg=colour114,none
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_attr(Some(""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_bg(Some(""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_fg(Some(""));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_activity_style(Some("reverse"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_bell_style(Some(""));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.window_status_content_style(Some(""));
    #[cfg(feature = "tmux_1_2")]
    let options = options.window_status_current_format(Some(" #I: #W #F "));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_attr(Some(""));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_bg(Some(""));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_fg(Some(""));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_current_style(Some(""));
    #[cfg(feature = "tmux_1_2")]
    let options = options.window_status_format(Some(" #I: #W #F "));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_last_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.window_status_separator(Some(" "));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_style(Some("fg=colour247,bg=#282c34,none"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.window_size(Some(WindowSize::Largest));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    let options = options.word_separators(Some(""));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let options = options.window_style(Some("fg=colour247,bg=colour238"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.wrap_search(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.xterm_keys(Some(Switch::On));

    let options = options.user_option("user-option-name", Some("value"));

    let options = options.options.to_string();

    let cmd = "set -g -w";
    let separator = " ; ";

    let mut origin = Vec::new();
    //cursor-colour none
    //cursor-style default
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "aggressive-resize", "off"));
    //allow-passthrough off
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {} {}", cmd, "allow-rename", "off"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {} {}", cmd, "alternate-screen", "on"));
    #[cfg(feature = "tmux_1_0")] // 0.8
    origin.push(format!("{} {} {}", cmd, "automatic-rename", "on"));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!(
        "{} {} {}",
        cmd,
        "automatic-rename-format",
        "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}"
    ));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {} {}", cmd, "c0-change-interval", ""));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {} {}", cmd, "c0-change-trigger", ""));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "clock-mode-colour", "colour135"));
    //copy-mode-match-style bg=cyan,fg=black
    //copy-mode-current-match-style bg=magenta,fg=black
    //copy-mode-mark-style bg=red,fg=black
    //fill-character ''
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "clock-mode-style", "24"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    origin.push(format!("{} {} {}", cmd, "force-height", "0"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    origin.push(format!("{} {} {}", cmd, "force-width", "0"));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    origin.push(format!("{} {} {}", cmd, "layout-history-limit", ""));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "main-pane-height", "24"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "main-pane-width", "80"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "mode-attr", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "mode-bg", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "mode-fg", ""));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "mode-keys", "vi"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    origin.push(format!("{} {} {}", cmd, "mode-mouse", ""));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!(
        "{} {} {}",
        cmd, "mode-style", "fg=colour196,bg=colour238,bold"
    ));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "monitor-activity", "off"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    origin.push(format!("{} {} {}", cmd, "monitor-content", ""));
    #[cfg(feature = "tmux_2_6")]
    origin.push(format!("{} {} {}", cmd, "monitor-bell", "on"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {} {}", cmd, "monitor-silence", "0"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {} {}", cmd, "other-pane-height", "0"));
    #[cfg(feature = "tmux_1_4")]
    origin.push(format!("{} {} {}", cmd, "other-pane-width", "0"));
    #[cfg(feature = "tmux_2_0")]
    origin.push(format!(
        "{} {} {}",
        cmd, "pane-active-border-style", "fg=colour114,bg=colour235"
    ));
    #[cfg(feature = "tmux_1_6")]
    origin.push(format!("{} {} {}", cmd, "pane-base-index", "0"));
    #[cfg(feature = "tmux_2_3")]
    origin.push(format!(
        "{} {} {}",
        cmd,
        "pane-border-format",
        "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\""
    ));
    //pane-border-indicators colour
    //pane-border-lines single
    #[cfg(feature = "tmux_2_3")]
    origin.push(format!("{} {} {}", cmd, "pane-border-status", "off"));
    //pane-colours
    //popup-style default
    //popup-border-style default
    //popup-border-lines single
    #[cfg(feature = "tmux_2_0")]
    origin.push(format!(
        "{} {} {}",
        cmd, "pane-border-style", "fg=colour238,bg=colour235"
    ));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    origin.push(format!("{} {} {}", cmd, "remain-on-exit", "off"));
    //remain-on-exit-format "Pane is dead (#{?#{!=:#{pane_dead_status},},status #{pane_dead_status},}#{?#{!=:#{pane_dead_signal},},signal #{pane_dead_signal},}, #{t:pane_dead_time})"
    //scroll-on-clear on
    //window-size latest
    //window-status-bell-style fg=colour253,bg=colour1,bold
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    origin.push(format!("{} {} {}", cmd, "synchronize-panes", "on"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    origin.push(format!("{} {} {}", cmd, "utf8", ""));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-active-style", "fg=colour253,bg=colour235"
    ));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-bell-attr", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-bell-bg", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-bell-fg", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-content-attr", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-content-bg", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-content-fg", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-activity-attr", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-activity-bg", ""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-activity-fg", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-attr", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-bg", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-fg", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-current-attr", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-current-bg", ""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-current-fg", ""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {} {}", cmd, "window-status-alert-attr", ""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {} {}", cmd, "window-status-alert-bg", ""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {} {}", cmd, "window-status-alert-fg", ""));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-status-activity-style", "reverse"
    ));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {} {}", cmd, "window-status-bell-style", ""));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-status-content-style", "fg=colour22,bg=colour114,none"
    ));
    #[cfg(feature = "tmux_1_2")]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-status-current-format", " #I: #W #F "
    ));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-last-attr", ""));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-last-bg", ""));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    origin.push(format!("{} {} {}", cmd, "window-status-last-fg", ""));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!("{} {} {}", cmd, "window-status-current-style", ""));
    #[cfg(feature = "tmux_1_2")]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-status-format", " #I: #W #F "
    ));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-status-last-style", "default"
    ));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {} {}", cmd, "window-status-separator", " "));
    #[cfg(feature = "tmux_1_9")]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-status-style", "fg=colour247,bg=#282c34,none"
    ));
    #[cfg(feature = "tmux_2_9")]
    origin.push(format!("{} {} {}", cmd, "window-size", "largest"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    origin.push(format!("{} {} {}", cmd, "word-separators", ""));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    origin.push(format!(
        "{} {} {}",
        cmd, "window-style", "fg=colour247,bg=colour238"
    ));
    #[cfg(feature = "tmux_1_7")]
    origin.push(format!("{} {} {}", cmd, "wrap-search", "on"));
    #[cfg(feature = "tmux_1_0")]
    origin.push(format!("{} {} {}", cmd, "xterm-keys", "on"));

    origin.push(format!("{} {} {}", cmd, "@user-option-name", "value"));

    let origin = origin.join(separator);

    assert_eq!(options, origin);
}
