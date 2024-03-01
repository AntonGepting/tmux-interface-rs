// Tests:
// * `::default()`
// * `.to_string()`
// * `::from_str()`

#[test]
fn default() {
    use crate::{
        Action, Activity, DestroyUnattached, DetachOnDestroy, SessionOptions, Status,
        StatusJustify, StatusKeys, StatusPosition, Switch,
    };

    let session_options = SessionOptions::new();

    let options = SessionOptions::default();
    #[cfg(feature = "tmux_2_6")]
    let options = options.activity_action(Some(Action::Other));
    #[cfg(feature = "tmux_1_8")]
    let options = options.assume_paste_time(Some(1));
    #[cfg(feature = "tmux_1_0")]
    let options = options.base_index(Some(0));
    #[cfg(feature = "tmux_1_0")]
    let options = options.bell_action(Some(Action::Any));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let options = options.bell_on_alert(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_4")))]
    let options = options.buffer_limit(Some(20));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_command(Some(""));
    #[cfg(feature = "tmux_1_0")]
    let options = options.default_shell(Some("/bin/bash"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.default_path(Some(""));
    #[cfg(feature = "tmux_2_9")]
    let options = options.default_size(Some((80, 24)));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_1")))]
    let options = options.default_terminal(Some("screen"));
    #[cfg(feature = "tmux_1_5")]
    let options = options.destroy_unattached(Some(DestroyUnattached::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.detach_on_destroy(Some(DetachOnDestroy::On));
    #[cfg(feature = "tmux_1_2")]
    let options = options.display_panes_active_colour(Some("red"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_colour(Some("blue"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_panes_time(Some(1000));
    #[cfg(feature = "tmux_1_0")]
    let options = options.display_time(Some(750));
    #[cfg(feature = "tmux_1_0")]
    let options = options.history_limit(Some(2000));
    #[cfg(feature = "tmux_2_2")]
    let options = options.key_table(Some("root"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.lock_after_time(Some(0));
    #[cfg(feature = "tmux_1_1")]
    let options = options.lock_command(Some("lock -np"));
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_2_1")))]
    let options = options.lock_server(Some(Switch::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_bg(Some("yellow"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_bg(Some("black"));
    #[cfg(all(feature = "tmux_1_6", not(feature = "tmux_1_9")))]
    let options = options.message_command_fg(Some("yellow"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.message_fg(Some("black"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_command_style(Some("bg=black,fg=yellow"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let options = options.message_limit(Some(20));
    #[cfg(feature = "tmux_1_9")]
    let options = options.message_style(Some("bg=yellow,fg=black"));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_resize_pane(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_pane(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_1")))]
    let options = options.mouse_select_window(Some(Switch::Off));
    #[cfg(feature = "tmux_2_1")]
    let options = options.mouse(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_2")))]
    let options = options.mouse_utf8(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_active_border_fg(Some("green"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_1_9")))]
    let options = options.pane_border_fg(Some("default"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_active_border_style(Some("fg=green"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.pane_border_style(Some("default"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.prefix(Some("C-b"));
    #[cfg(feature = "tmux_1_6")]
    let options = options.prefix2(Some("Invalid#1fff00000000")); // KEYC_NONE = 0xfff
    #[cfg(feature = "tmux_1_7")]
    let options = options.renumber_windows(Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.repeat_time(Some(500));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let options = options.set_remain_on_exit(Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles(Some(Switch::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.set_titles_string(Some("#S:#I:#W - \"#T\" #{session_alerts}"));
    #[cfg(feature = "tmux_2_6")]
    let options = options.silence_action(Some(Action::Other));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status(Some(Status::On));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_bg(Some("green"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_fg(Some("black"));
    #[cfg(feature = "tmux_2_9")]
    let options = options.status_format(Some(vec!["#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]",
    "#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }"
    ]));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_interval(Some(15));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_justify(Some(StatusJustify::Left));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_keys(Some(StatusKeys::Emacs));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_3_2")))]
    let options = options.status_left(Some("[#S] "));
    #[cfg(feature = "tmux_3_2")]
    let options = options.status_left(Some("[#{session_name}] "));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_left_fg(Some("default"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_left_length(Some(10));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_left_style(Some("default"));
    #[cfg(feature = "tmux_1_7")]
    let options = options.status_position(Some(StatusPosition::Bottom));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right(Some("#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\"#{=21:pane_title}\" %H:%M %d-%b-%y"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_attr(Some("none"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_bg(Some("default"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_1_9")))]
    let options = options.status_right_fg(Some("default"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.status_right_length(Some(40));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_right_style(Some("default"));
    #[cfg(feature = "tmux_1_9")]
    let options = options.status_style(Some("bg=green,fg=black"));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_2")))]
    let options = options.status_utf8(Some(Switch::Off));
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_1_9")))]
    let options = options.terminal_overrides(Some("*88col*:colors=88,*256col*:colors=256xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cc=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Cs=\\E[%p1%d q:Csr=\\E[2 q"));
    #[cfg(all(feature = "tmux_1_9", not(feature = "tmux_2_0")))]
    let options = options.terminal_overrides(Some("*256col*:colors=256,xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT"));
    // #[cfg(feature = "tmux_2_0")]
    // let options = options.terminal_overrides(Some("xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q,screen*:XT"));
    #[cfg(feature = "tmux_1_0")]
    let options = options.update_environment(Some(vec![
        "DISPLAY",
        "KRB5CCNAME",
        "SSH_ASKPASS",
        "SSH_AUTH_SOCK",
        "SSH_AGENT_PID",
        "SSH_CONNECTION",
        "WINDOWID",
        "XAUTHORITY",
    ]));
    #[cfg(all(feature = "tmux_2_6", not(feature = "tmux_3_0")))]
    let options = options.user_keys(None::<[&str; 0]>);
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_activity(Some(Activity::Off));
    #[cfg(feature = "tmux_1_0")]
    let options = options.visual_bell(Some(Activity::Off));
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_0")))]
    let options = options.visual_content(Some(Switch::Off));
    #[cfg(feature = "tmux_1_4")]
    let options = options.visual_silence(Some(Activity::Off));
    #[cfg(feature = "tmux_1_6")]
    let options = options.word_separators(Some(" "));

    assert_eq!(session_options, options);
}

#[test]
fn to_stirng() {}

#[test]
fn from_str() {
    use crate::SessionOptions;

    let session_options_str = r#"
activity-action other
assume-paste-time 1
base-index 1
bell-action none
default-command ""
default-shell "/usr/bin/fish"
destroy-unattached off
detach-on-destroy on
display-panes-active-colour red
display-panes-colour blue
display-panes-time 1000
display-time 750
history-limit 2000
key-table "root"
lock-after-time 0
lock-command "lock -np"
message-command-style fg=blue,bg=black
message-style fg=colour232,bg=colour166,bright
mouse on
prefix C-a
prefix2 None
renumber-windows off
repeat-time 500
set-titles on
set-titles-string ""
silence-action other
status on
status-interval 2
status-justify left
status-keys emacs
status-left ""
status-left-length 50
status-left-style default
status-position bottom
status-right ""
status-right-length 50
status-right-style default
status-style fg=colour247,bg=#282c34
visual-activity off
visual-bell off
visual-silence off
word-separators ""
"#;
    let session_options = session_options_str.parse::<SessionOptions>().unwrap();
    dbg!(&session_options);
}
