#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
use crate::ModeMouse;

#[test]
fn set_global_window_options() {
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    use crate::ModeMouse;
    #[cfg(feature = "tmux_2_3")]
    use crate::PaneBorderStatus;
    #[cfg(feature = "tmux_2_9")]
    use crate::WindowSize;
    use crate::{
        ClockModeStyle, SetGlobalWindowOptions, SetUserOptions, SetWindowOptions, StatusKeys,
        Switch,
    };

    let options = SetGlobalWindowOptions::new();
    let target = ":";

    //cursor-colour none
    //cursor-style default
    #[cfg(feature = "tmux_1_0")]
    let options = options.aggressive_resize(Some(target), Some(Switch::Off));
    //allow-passthrough off
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_3_0")))]
    let options = options.allow_rename(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
    let options = options.alternate_screen(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_0")] // 0.8
    let options = options.automatic_rename(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_9")]
    let options = options.automatic_rename_format(
        Some(target),
        Some("#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}".to_string()),
    );
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let options = options.c0_change_interval(Some(target), Some(0));
    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_2_1")))]
    let options = options.c0_change_trigger(Some(target), Some(""));
    #[cfg(feature = "tmux_1_0")]
    let options = options.clock_mode_colour(Some(target), Some("colour135"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.clock_mode_style(Some(target), Some(ClockModeStyle::_24));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let options = options.force_height(Some(target), Some(0));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_9")))]
    let options = options.force_width(Some(target), Some(0));
    //copy-mode-match-style bg=cyan,fg=black
    //copy-mode-current-match-style bg=magenta,fg=black
    //copy-mode-mark-style bg=red,fg=black
    //fill-character ''

    #[cfg(all(feature = "tmux_1_7", not(feature = "tmux_1_8")))]
    let options = options.layout_history_limit(Some(target), Some(0));
    #[cfg(feature = "tmux_1_0")]
    let options = options.main_pane_height(Some(target), Some(24));
    #[cfg(feature = "tmux_1_0")]
    let options = options.main_pane_width(Some(target), Some(80));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_attr(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_bg(Some(target));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.mode_fg(Some(target));
    #[cfg(feature = "tmux_1_0")]
    let options = options.mode_keys(Some(target), Some(StatusKeys::Vi));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.mode_mouse(Some(target), Some(ModeMouse::Off));
    #[cfg(feature = "tmux_1_9")]
    let options = options.mode_style(Some(target), Some("fg=colour196,bg=colour238,bold"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.monitor_activity(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.monitor_content(Some(target));
    #[cfg(feature = "tmux_2_6")]
    let options = options.monitor_bell(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_4")]
    let options = options.monitor_silence(Some(target), Some(0));

    #[cfg(feature = "tmux_1_4")]
    let options = options.other_pane_height(Some(target), Some(0));
    #[cfg(feature = "tmux_1_4")]
    let options = options.other_pane_width(Some(target), Some(0));
    #[cfg(feature = "tmux_2_0")]
    let options = options.pane_active_border_style(Some(target), Some("fg=colour114,bg=colour235"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.pane_base_index(Some(target), Some(0));
    #[cfg(feature = "tmux_2_3")]
    let options = options.pane_border_format(
        Some(target),
        Some("#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\""),
    );
    #[cfg(feature = "tmux_2_3")]
    let options = options.pane_border_status(Some(target), Some(PaneBorderStatus::Off));
    //pane-border-indicators colour
    //pane-border-lines single
    //pane-colours
    #[cfg(feature = "tmux_2_0")]
    let options = options.pane_border_style(Some(target), Some("fg=colour238,bg=colour235"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_0")))]
    let options = options.remain_on_exit(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_2")))]
    let options = options.synchronize_panes(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.utf8(Some(target), Some(Switch::Off));
    //popup-style default
    //popup-border-style default
    //popup-border-lines single
    //remain-on-exit-format "Pane is dead (#{?#{!=:#{pane_dead_status},},status #{pane_dead_status},}#{?#{!=:#{pane_dead_signal},},signal #{pane_dead_signal},}, #{t:pane_dead_time})"
    //scroll-on-clear on
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let options = options.window_active_style(Some(target), Some("fg=colour253,bg=colour235"));
    //window-status-activity-style reverse
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_bell_fg(Some(target), Some(""));
    //window-status-bell-style fg=colour253,bg=colour1,bold
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_content_fg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.window_status_activity_fg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_fg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.window_status_current_fg(Some(target), Some(""));
    //window-status-current-format " #I: #W #F "
    //window-status-current-style fg=colour22,bg=colour114,none
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_6")))]
    let options = options.window_status_alert_fg(Some(target), Some(""));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_activity_style(Some(target), Some("reverse"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_bell_style(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.window_status_content_style(Some(target), Some(""));
    #[cfg(feature = "tmux_1_2")]
    let options = options.window_status_current_format(Some(target), Some(" #I: #W #F "));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_attr(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_bg(Some(target), Some(""));
    #[cfg(all(feature = "tmux_1_8", not(feature = "tmux_1_9")))]
    let options = options.window_status_last_fg(Some(target), Some(""));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_current_style(Some(target), Some(""));
    #[cfg(feature = "tmux_1_2")]
    let options = options.window_status_format(Some(target), Some(" #I: #W #F "));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_last_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.window_status_separator(Some(target), Some(" "));
    #[cfg(feature = "tmux_1_9")]
    let options = options.window_status_style(Some(target), Some("fg=colour247,bg=#282c34,none"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.window_size(Some(target), Some(WindowSize::Largest));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_6")))]
    let options = options.word_separators(Some(target), Some(""));
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0")))]
    let options = options.window_style(Some(target), Some("fg=colour247,bg=colour238"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.wrap_search(Some(target), Some(Switch::On));
    #[cfg(feature = "tmux_1_0")]
    let options = options.xterm_keys(Some(target), Some(Switch::On));

    let options = options.user_option(Some(target), "user-option-name", Some("value"));

    let options = options.options.to_string();

    let cmd = "set -g -w";
    let cmd = format!("{} -t {}", cmd, target);
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
