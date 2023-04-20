#[test]
fn set_global_session_options_tests() {
    use crate::{
        Action, Activity, DetachOnDestroy, SetGlobalSessionOptions, SetSessionOptionsTr,
        SetUserOptions, Status, StatusJustify, StatusKeys, StatusPosition, Switch,
    };

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let target = ":";
    let cmd = format!("{} -g -t {}", cmd, target);

    let mut v = Vec::new();

    #[cfg(feature = "tmux_2_6")]
    v.push(format!("{} {} {}", cmd, "activity-action", "other"));
    #[cfg(feature = "tmux_1_8")]
    v.push(format!("{} {} {}", cmd, "assume-paste-time", "1"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "base-index", "0"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "bell-action", "any"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    v.push(format!("{} {} {}", cmd, "bell-on-alert", "off"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    v.push(format!("{} {} {}", cmd, "buffer-limit", "20"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-command", ""));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "default-shell", "/bin/bash"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "default-path", ""));
    #[cfg(feature = "tmux_2_9")]
    v.push(format!("{} {} {}", cmd, "default-size", "80x24"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    v.push(format!("{} {} {}", cmd, "default-terminal", "screen"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "destroy-unattached", "off"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "detach-on-destroy", "on"));
    #[cfg(feature = "tmux_1_2")]
    v.push(format!(
        "{} {} {}",
        cmd, "display-panes-active-colour", "red"
    ));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "display-panes-colour", "blue"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "display-panes-time", "1000"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "display-time", "750"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "history-limit", "2000"));
    #[cfg(feature = "tmux_2_2")]
    v.push(format!("{} {} {}", cmd, "key-table", "root"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "lock-after-time", "0"));
    #[cfg(feature = "tmux_1_1")]
    v.push(format!("{} {} {}", cmd, "lock-command", "lock -np"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    v.push(format!("{} {} {}", cmd, "lock-server", "on"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "message-attr", "none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "message-bg", "yellow"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "message-command-attr", "none"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "message-command-bg", "black"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "message-command-fg", "yellow"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "message-fg", ""));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-command-style", "bg=black,fg=yellow"
    ));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    v.push(format!("{} {} {}", cmd, "message-limit", "20"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "message-style", "bg=yellow,fg=black"
    ));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    v.push(format!("{} {} {}", cmd, "mouse-resize-pane", "off"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    v.push(format!("{} {} {}", cmd, "mouse-select-pane", "off"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    v.push(format!("{} {} {}", cmd, "mouse-select-window", "off"));
    #[cfg(feature = "tmux_2_1")]
    v.push(format!("{} {} {}", cmd, "mouse", "off"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    v.push(format!("{} {} {}", cmd, "mouse-utf8", "off"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "pane-active-border-bg", "default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "pane-active-border-fg", "green"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "pane-border-bg", "default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "pane-border-fg", "fg=green"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    v.push(format!(
        "{} {} {}",
        cmd, "pane-active-border-style", "fg=green"
    ));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    v.push(format!("{} {} {}", cmd, "pane-border-style", "default"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "prefix", "C-b"));
    #[cfg(feature = "tmux_1_6")]
    v.push(format!("{} {} {}", cmd, "prefix2", "Invalid#1fff00000000"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "renumber-windows", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "repeat-time", "500"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    v.push(format!("{} {} {}", cmd, "set-remain-on-exit", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "set-titles", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!(
        "{} {} {}",
        cmd, "set-titles-string", "#S:#I:#W - \"#T\" #{session_alerts}"
    ));
    #[cfg(feature = "tmux_2_6")]
    v.push(format!("{} {} {}", cmd, "silence-action", "other"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status", "on"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-attr", "none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-bg", "green"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-fg", "black"));
    #[cfg(feature = "tmux_2_9")]
    {
        v.push(format!("{} {} {}", cmd, "status-format[0]", "#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]"));
        v.push(format!("{} {} {}", cmd, "status-format[1]", "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"));
    }
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-interval", "15"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-justify", "left"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-keys", "emacs"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-left", "[#S] "));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-left-attr", "none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-left-bg", "default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-left-fg", "default"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-left-length", "10"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-left-style", "default"));
    #[cfg(feature = "tmux_1_7")]
    v.push(format!("{} {} {}", cmd, "status-position", "bottom"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right", "#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\"#{=21:pane_title}\" %H:%M %d-%b-%y"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-right-attr", "none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-right-bg", "default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    v.push(format!("{} {} {}", cmd, "status-right-fg", "default"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "status-right-length", "40"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!("{} {} {}", cmd, "status-right-style", "default"));
    #[cfg(feature = "tmux_1_9")]
    v.push(format!(
        "{} {} {}",
        cmd, "status-style", "bg=green,fg=black"
    ));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    v.push(format!("{} {} {}", cmd, "status-utf8", "off"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    v.push(format!("{} {} {}", cmd, "terminal-overrides", "*256col*:colors=256,xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT"));
    #[cfg(feature = "tmux_1_0")]
    {
        v.push(format!("{} {} {}", cmd, "update-environment[0]", "DISPLAY"));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[1]", "KRB5CCNAME"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[2]", "SSH_ASKPASS"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[3]", "SSH_AUTH_SOCK"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[4]", "SSH_AGENT_PID"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[5]", "SSH_CONNECTION"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[6]", "WINDOWID"
        ));
        v.push(format!(
            "{} {} {}",
            cmd, "update-environment[7]", "XAUTHORITY"
        ));
    }
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "visual-activity", "off"));
    #[cfg(feature = "tmux_1_0")]
    v.push(format!("{} {} {}", cmd, "visual-bell", "off"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    v.push(format!("{} {} {}", cmd, "visual-content", "off"));
    #[cfg(feature = "tmux_1_4")]
    v.push(format!("{} {} {}", cmd, "visual-silence", "off"));
    #[cfg(feature = "tmux_1_6")]
    v.push(format!("{} {} {}", cmd, "word-separators", " "));
    v.push(format!("{} {} {}", cmd, "@user-option-name", "value"));
    let origin = v.join(" ; ");

    let options = SetGlobalSessionOptions::new();

    #[cfg(feature = "tmux_2_6")]
    let options = options.activity_action(Some(target), Some(Action::Other));
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time(Some(target), Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index(Some(target), Some(0));
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action(Some(target), Some(Action::Any));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit(Some(target), Some(20));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command(Some(target), Some(""));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell(Some(target), Some("/bin/bash"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path(Some(target), Some(""));
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size(Some(target), Some((80, 24)));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal(Some(target), Some("screen"));
    #[cfg(feature = "tmux_1_4")]
    let options = options.destroy_unattached(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy(Some(target), Some(DetachOnDestroy::On));
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour(Some(target), Some("red"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour(Some(target), Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time(Some(target), Some(1000));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time(Some(target), Some(750));
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit(Some(target), Some(2000));
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table(Some(target), Some("root"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time(Some(target), Some(0));
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command(Some(target), Some("lock -np"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server(Some(target), Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr(Some(target), Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg(Some(target), Some("yellow"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr(Some(target), Some("none"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg(Some(target), Some("black"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg(Some(target), Some("yellow"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg(Some(target), Some(""));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style(Some(target), Some("bg=black,fg=yellow"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit(Some(target), Some(20));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style(Some(target), Some("bg=yellow,fg=black"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg(Some(target), Some("default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg(Some(target), Some("green"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg(Some(target), Some("default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg(Some(target), Some("fg=green"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style(Some(target), Some("fg=green"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix(Some(target), Some("C-b"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2(Some(target), Some("Invalid#1fff00000000"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time(Some(target), Some(500));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options =
        options.set_titles_string(Some(target), Some("#S:#I:#W - \"#T\" #{session_alerts}"));
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action(Some(target), Some(Action::Other));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status(Some(target), Some(Status::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr(Some(target), Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg(Some(target), Some("green"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg(Some(target), Some("black"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format(
        Some(target),
        Some(vec![
            "#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]",
            "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"
    ]));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval(Some(target), Some(15));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify(Some(target), Some(StatusJustify::Left));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys(Some(target), Some(StatusKeys::Emacs));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left(Some(target), Some("[#S] "));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr(Some(target), Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg(Some(target), Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length(Some(target), Some(10));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position(Some(target), Some(StatusPosition::Bottom));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right(Some(target), Some("#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\"#{=21:pane_title}\" %H:%M %d-%b-%y"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr(Some(target), Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg(Some(target), Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length(Some(target), Some(40));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style(Some(target), Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style(Some(target), Some("bg=green,fg=black"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8(Some(target), Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides(Some(target), Some("*256col*:colors=256,xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.update_environment(
        Some(target),
        Some(vec![
            "DISPLAY",
            "KRB5CCNAME",
            "SSH_ASKPASS",
            "SSH_AUTH_SOCK",
            "SSH_AGENT_PID",
            "SSH_CONNECTION",
            "WINDOWID",
            "XAUTHORITY",
        ]),
    );
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity(Some(target), Some(Activity::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell(Some(target), Some(Activity::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content(Some(target), Some(Switch::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence(Some(target), Some(Activity::Off));
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators(Some(target), Some(" "));
    let options = options.user_option_ext(Some(target), "user-option-name", Some("value"));

    let options = options.build().to_string();

    assert_eq!(origin, options);
}
