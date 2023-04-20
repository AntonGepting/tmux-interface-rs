// Tests:
// * `::default()`
// * `.to_string()`
// * `::from_str()`

#[test]
fn default() {
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    use crate::ModeMouse;
    #[cfg(feature = "tmux_2_3")]
    use crate::PaneBorderStatus;
    #[cfg(feature = "tmux_2_9")]
    use crate::WindowSize;
    use crate::{ClockModeStyle, StatusKeys, Switch, WindowOptions};

    let default_window_options = WindowOptions::new();
    let window_options = WindowOptions::default();

    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.aggressive_resize(Some(Switch::Off));
    #[cfg(all(feature = "tmux_2_7", not(feature = "tmux_3_0")))]
    let window_options = window_options.allow_rename(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_6")))]
    let window_options = window_options.allow_rename(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    let window_options = window_options.alternate_screen(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")] // 0.8
    let window_options = window_options.automatic_rename(Some(Switch::On));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.automatic_rename_format(Some(
        "#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}",
    ));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let window_options = window_options.c0_change_interval(Some(100));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let window_options = window_options.c0_change_trigger(Some(250));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.clock_mode_colour(Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.clock_mode_style(Some(ClockModeStyle::H24));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let window_options = window_options.force_height(Some(0));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let window_options = window_options.force_width(Some(0));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    let window_options = window_options.layout_history_limit(Some(20));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.main_pane_height(Some(24));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.main_pane_width(Some(80));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.mode_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.mode_bg(Some("yellow"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.mode_fg(Some("black"));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.mode_keys(Some(StatusKeys::Emacs));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let window_options = window_options.mode_mouse(Some(ModeMouse::Off));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.mode_style(Some("bg=yellow,fg=black"));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.monitor_activity(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let window_options = window_options.monitor_content(Some(""));
    #[cfg(feature = "tmux_2_6")]
    let window_options = window_options.monitor_bell(Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let window_options = window_options.monitor_silence(Some(0));
    #[cfg(feature = "tmux_1_4")]
    let window_options = window_options.other_pane_height(Some(0));
    #[cfg(feature = "tmux_1_4")]
    let window_options = window_options.other_pane_width(Some(0));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_3_2")))]
    let window_options = window_options.pane_active_border_style(Some("fg=green"));
    #[cfg(feature = "tmux_3_2")]
    let window_options = window_options.pane_active_border_style(Some(
        "#{?pane_in_mode,fg=yellow,#{?synchronize-panes,fg=red,fg=green}}",
    ));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.pane_active_border_bg(Some("default"));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.pane_active_border_fg(Some("green"));
    #[cfg(feature = "tmux_1_6")]
    let window_options = window_options.pane_base_index(Some(0));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.pane_border_bg(Some("default"));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.pane_border_fg(Some("default"));
    #[cfg(feature = "tmux_2_3")]
    let window_options = window_options.pane_border_format(Some(
        "#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    ));
    #[cfg(feature = "tmux_2_3")]
    let window_options = window_options.pane_border_status(Some(PaneBorderStatus::Off));
    #[cfg(feature = "tmux_2_0")]
    let window_options = window_options.pane_border_style(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    let window_options = window_options.remain_on_exit(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    let window_options = window_options.synchronize_panes(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let window_options = window_options.utf8(Some(Switch::Off));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let window_options = window_options.window_active_style(Some("default"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bell_attr(Some("reverse"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bell_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bell_fg(Some("default"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_content_attr(Some("reverse"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_content_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_content_fg(Some("default"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_activity_attr(Some("reverse"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_activity_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_activity_fg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_fg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_current_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_current_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_current_fg(Some("default"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let window_options = window_options.window_status_alert_attr(Some("reverse"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let window_options = window_options.window_status_alert_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let window_options = window_options.window_status_alert_fg(Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_activity_style(Some("reverse"));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_bell_style(Some("reverse"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let window_options = window_options.window_status_content_style(Some("reverse"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
    let window_options = window_options.window_status_current_format(Some("#I:#W#F"));
    #[cfg(feature = "tmux_2_1")]
    let window_options = window_options
        .window_status_current_format(Some("#I:#W#{?window_flags,#{window_flags}, }"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_last_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_last_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let window_options = window_options.window_status_last_fg(Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_current_style(Some("default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
    let window_options = window_options.window_status_format(Some("#I:#W#F"));
    #[cfg(feature = "tmux_2_1")]
    let window_options =
        window_options.window_status_format(Some("#I:#W#{?window_flags,#{window_flags}, }"));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_last_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let window_options = window_options.window_status_separator(Some("\" \""));
    #[cfg(feature = "tmux_1_9")]
    let window_options = window_options.window_status_style(Some("default"));
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_1")))]
    let window_options = window_options.window_size(Some(WindowSize::Smallest));
    #[cfg(feature = "tmux_3_1")]
    let window_options = window_options.window_size(Some(WindowSize::Latest));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    let window_options = window_options.word_separators(Some(" -_@"));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let window_options = window_options.window_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let window_options = window_options.wrap_search(Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let window_options = window_options.xterm_keys(Some(Switch::On));

    assert_eq!(default_window_options, window_options);
}

#[test]
fn to_string() {
    use crate::WindowOptions;

    let window_options = WindowOptions::new().to_string();

    let mut v = Vec::new();

    #[cfg(feature = "tmux_1_0")]
    v.push("aggressive-resize off");
    // allow-passthrough off
    #[cfg(all(feature = "tmux_2_7", not(feature = "tmux_3_0")))]
    v.push("allow-rename off");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_6")))]
    v.push("allow-rename on");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    v.push("alternate-screen on");
    #[cfg(feature = "tmux_1_0")] // 0.8
    v.push("automatic-rename on");
    #[cfg(feature = "tmux_1_9")]
    v.push("automatic-rename-format #{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    v.push("c0-change-interval 100");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    v.push("c0-change-trigger 250");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-colour blue");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-style 24");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-height 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-width 0");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    v.push("layout-history-limit 0");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-height 24");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-width 80");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-attr 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-bg 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-fg 0");
    #[cfg(feature = "tmux_1_0")]
    v.push("mode-keys emacs");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    v.push("mode-mouse off");
    #[cfg(feature = "tmux_1_9")]
    v.push("mode-style bg=yellow,fg=black");
    #[cfg(feature = "tmux_1_0")]
    v.push("monitor-activity off");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    v.push("monitor-content ");
    #[cfg(feature = "tmux_2_6")]
    v.push("monitor-bell on");
    #[cfg(feature = "tmux_1_4")]
    v.push("monitor-silence 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-height 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-width 0");
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_3_2")))]
    v.push("pane-active-border-style fg=green");
    #[cfg(feature = "tmux_3_2")]
    v.push(
        "pane-active-border-style #{?pane_in_mode,fg=yellow,#{?synchronize-panes,fg=red,fg=green}}",
    );
    #[cfg(feature = "tmux_1_6")]
    v.push("pane-base-index 0");
    #[cfg(feature = "tmux_2_3")]
    v.push(
        "pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    );
    #[cfg(feature = "tmux_2_3")]
    v.push("pane-border-status off");
    #[cfg(feature = "tmux_2_0")]
    v.push("pane-border-style default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    v.push("remain-on-exit off");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    v.push("synchronize-panes off");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    v.push("utf8 off");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-active-style default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-attr reverse");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-bg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-fg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-attr reverse");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-bg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-fg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-attr reverse");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-bg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-fg default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-attr none");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-bg default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-fg default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-current-attr reverse");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-current-bg default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-current-fg default");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-attr default");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-bg default");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-fg default");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-activity-style reverse");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-bell-style reverse");
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    v.push("window-status-content-style reverse");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
    v.push("window-status-current-format #I:#W#F");
    #[cfg(feature = "tmux_2_1")]
    v.push("window-status-current-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-attr none");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-bg default");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-fg default");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-current-style default");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
    v.push("window-status-format #I:#W#F");
    #[cfg(feature = "tmux_2_1")]
    v.push("window-status-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-attr none");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-last-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("window-status-separator \" \"");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-style default");
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_1")))]
    v.push("window-size smallest");
    #[cfg(feature = "tmux_3_1")]
    v.push("window-size latest");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    v.push("word-separators \"-_@\"");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("wrap-search on");
    #[cfg(feature = "tmux_1_0")]
    v.push("xterm-keys on");

    let origin = v.join("\n");

    assert_eq!(origin, window_options);
}

#[test]
fn from_str() {
    use crate::WindowOptions;

    let mut v = Vec::new();
    #[cfg(feature = "tmux_1_0")]
    v.push("aggressive-resize off");
    #[cfg(all(feature = "tmux_2_7", not(feature = "tmux_3_0")))]
    v.push("allow-rename off");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_2_6")))]
    v.push("allow-rename on");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    v.push("alternate-screen on");
    #[cfg(feature = "tmux_1_0")]
    v.push("automatic-rename on");
    #[cfg(feature = "tmux_1_9")]
    v.push("automatic-rename-format #{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    v.push("c0-change-interval 100");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    v.push("c0-change-trigger 250");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-colour blue");
    #[cfg(feature = "tmux_1_0")]
    v.push("clock-mode-style 24");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-height 0");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    v.push("force-width 0");
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    v.push("layout-history-limit 20");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-height 24");
    #[cfg(feature = "tmux_1_0")]
    v.push("main-pane-width 80");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-attr none");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-bg yellow");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("mode-fg black");
    #[cfg(feature = "tmux_1_0")]
    v.push("mode-keys emacs");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    v.push("mode-mouse off");
    #[cfg(feature = "tmux_1_9")]
    v.push("mode-style bg=yellow,fg=black");
    #[cfg(feature = "tmux_1_0")]
    v.push("monitor-activity off");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    v.push("monitor-content  ");
    #[cfg(feature = "tmux_2_6")]
    v.push("monitor-bell on");
    #[cfg(feature = "tmux_1_4")]
    v.push("monitor-silence 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-height 0");
    #[cfg(feature = "tmux_1_4")]
    v.push("other-pane-width 0");
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_3_2")))]
    v.push("pane-active-border-style fg=green");
    #[cfg(feature = "tmux_3_2")]
    v.push(
        "pane-active-border-style #{?pane_in_mode,fg=yellow,#{?synchronize-panes,fg=red,fg=green}}",
    );
    #[cfg(feature = "tmux_1_6")]
    v.push("pane-base-index 0");
    #[cfg(feature = "tmux_2_3")]
    v.push(
        "pane-border-format #{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"",
    );
    #[cfg(feature = "tmux_2_3")]
    v.push("pane-border-status off");
    #[cfg(feature = "tmux_2_0")]
    v.push("pane-border-style default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    v.push("remain-on-exit off");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    v.push("synchronize-panes off");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    v.push("utf8 off");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-active-style default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-attr reverse");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-bg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-bell-fg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-attr reverse");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-bg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-content-fg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-attr reverse");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-bg default");
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push("window-status-activity-fg default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-attr none");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-bg default");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push("window-status-fg default");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-attr reverse");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-bg default");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    v.push("window-status-alert-fg default");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-activity-style reverse");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-bell-style reverse");
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    v.push("window-status-content-style reverse");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
    v.push("window-status-current-format #I:#W#F");
    #[cfg(feature = "tmux_2_1")]
    v.push("window-status-current-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-attr none");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-bg default");
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    v.push("window-status-last-fg default");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-current-style default");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_1")))]
    v.push("window-status-format #I:#W#F");
    #[cfg(feature = "tmux_2_1")]
    v.push("window-status-format #I:#W#{?window_flags,#{window_flags}, }");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-last-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("window-status-separator \" \"");
    #[cfg(feature = "tmux_1_9")]
    v.push("window-status-style default");
    #[cfg(all(feature = "tmux_2_9", not(feature = "tmux_3_1")))]
    v.push("window-size smallest");
    #[cfg(feature = "tmux_3_1")]
    v.push("window-size latest");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    v.push("word-separators -_@");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    v.push("window-style default");
    #[cfg(feature = "tmux_1_7")]
    v.push("wrap-search on");
    #[cfg(feature = "tmux_1_0")]
    v.push("xterm-keys on");

    let window_options_str = v.join("\n");
    let window_options = window_options_str.parse::<WindowOptions>().unwrap();

    // FIXME test bypass
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let window_options = window_options.monitor_content(Some(""));

    let default_window_options = WindowOptions::new();

    assert_eq!(window_options, default_window_options);
}
