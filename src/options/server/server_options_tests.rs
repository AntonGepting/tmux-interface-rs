#[test]
fn show_generated_struct() {
    use crate::ServerOptions;

    let _server_options = ServerOptions {
        ..Default::default()
    };
    //dbg!(_server_options);
}

//#[test]
//fn bitflags() {
//use crate::{SERVER_OPTIONS_ALL, SERVER_OPTIONS_NONE};
//let bitflags =
//// 15_____8_7______0
//0b_11111111_11111111;
////println!("{:b}", SERVER_OPTIONS_ALL);
////println!("{:b}", &bitflags);
//assert_eq!(bitflags, SERVER_OPTIONS_ALL);
//assert_eq!(0, SERVER_OPTIONS_NONE);
//}

// FIXME: conditionals
#[test]
fn parse() {
    #[cfg(feature = "tmux_2_6")]
    use crate::SetClipboard;
    use crate::{ServerOptions, Switch};

    let mut options = ServerOptions::new();
    let options = options.buffer_limit(50);
    #[cfg(feature = "tmux_2_1")]
    let options = options.default_terminal("\"screen-256color\"");
    #[cfg(feature = "tmux_2_7")]
    let options = options.exit_empty(Switch::On);
    #[cfg(feature = "tmux_2_4")]
    let options = options.command_alias(vec![
        "\"split-pane=split-window\"",
        "\"splitp=split-window\"",
    ]);
    let server_options_default = options;

    // test int, string, enum, vec
    let server_options_str = r#"buffer-limit 50
default-terminal "screen-256color"
exit-empty on
command-alias[0] "split-pane=split-window"
command-alias[1] "splitp=split-window"
"#;
    let server_options = server_options_str.parse::<ServerOptions>().unwrap();
    assert_eq!(server_options_default, server_options);

    let mut options = ServerOptions::new();
    let options = options.buffer_limit(50);
    #[cfg(feature = "tmux_2_4")]
    let options = options.command_alias(vec![
        "\"split-pane=split-window\"",
        "\"splitp=split-window\"",
        "\"server-info=show-messages -JT\"",
        "\"info=show-messages -JT\"",
        "\"choose-window=choose-tree -w\"",
        "\"choose-session=choose-tree -s\"",
    ]);
    #[cfg(feature = "tmux_2_1")]
    let options = options.default_terminal("\"screen-256color\"");
    let options = options.escape_time(500);
    #[cfg(feature = "tmux_2_7")]
    let options = options.exit_empty(Switch::On);
    let options = options.exit_unattached(Switch::Off);
    #[cfg(feature = "tmux_1_9")]
    let options = options.focus_events(Switch::Off);
    #[cfg(feature = "tmux_2_1")]
    let options = options.history_file("\"\"");
    #[cfg(feature = "tmux_2_0")]
    let options = options.message_limit(100);
    #[cfg(feature = "tmux_2_6")]
    let options = options.set_clipboard(SetClipboard::External);
    #[cfg(feature = "tmux_2_0")]
    let options = options.terminal_overrides(vec!["\"xterm*:XT:Ms=\\\\E]52;%p1%s;%p2%s\\\\007:Cs=\\\\E]12;%p1%s\\\\007:Cr=\\\\E]112\\\\007:Ss=\\\\E[%p1%d q:Se=\\\\E[2 q\"",
"\"screen*:XT\""]);
    //#[cfg(feature = "tmux_3_0")]
    //builder.user_keys = None;

    let server_options_default = options;

    let server_options_str = r#"buffer-limit 50
command-alias[0] "split-pane=split-window"
command-alias[1] "splitp=split-window"
command-alias[2] "server-info=show-messages -JT"
command-alias[3] "info=show-messages -JT"
command-alias[4] "choose-window=choose-tree -w"
command-alias[5] "choose-session=choose-tree -s"
default-terminal "screen-256color"
escape-time 500
exit-empty on
exit-unattached off
focus-events off
history-file ""
message-limit 100
set-clipboard external
terminal-overrides[0] "xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q"
terminal-overrides[1] "screen*:XT"
user-keys
"#;
    let server_options = server_options_str.parse::<ServerOptions>().unwrap();
    assert_eq!(server_options_default, server_options);

    let server_options_default = ServerOptions {
        #[cfg(feature = "tmux_2_1")]
        history_file: Some("\"\"".into()),
        ..Default::default()
    };
    let server_options_str = "history-file \"\"";
    let server_options = server_options_str.parse::<ServerOptions>().unwrap();
    assert_eq!(server_options_default, server_options);
}

#[test]
fn to_string() {
    //use crate::ServerOptionsBuilder;
    use crate::ServerOptions;

    let server_options = ServerOptions::new().buffer_limit(50);
    assert_eq!(server_options.to_string(), "buffer-limit 50");
}

//#[test]
//fn get_all() {
//use crate::ServerOptions;

//let server_options = ServerOptions::get().unwrap();
//dbg!(server_options);
//}

//#[test]
//fn get_single() {
//use crate::ServerOptions;
//use crate::ESCAPE_TIME;
//#[cfg(feature = "tmux_1_7")]
//let _server_options = ServerOptions::get(ESCAPE_TIME).unwrap();
////assert_eq!(server_options.escape_time, Some(500));
//}

//#[test]
//fn set_single() {
//use crate::common::server_options::ESCAPE_TIME;
//use crate::{ServerOptions, ServerOptionsBuilder};

//let server_options = ServerOptionsBuilder::new().escape_time(600).build();
//server_options.set(ESCAPE_TIME).unwrap();
//let server_options = ServerOptions::get(ESCAPE_TIME).unwrap();
//assert_eq!(server_options.escape_time, Some(600));

//let server_options = ServerOptionsBuilder::new().escape_time(500).build();
//server_options.set(ESCAPE_TIME).unwrap();
//}

#[test]
fn server_options_from_str() {
    use crate::{ServerOptions, SetClipboard, Switch};

    let mut v = Vec::new();

    #[cfg(feature = "tmux_3_1")]
    v.push("backspace C-?");
    #[cfg(feature = "tmux_1_5")]
    v.push("buffer-limit 50");
    #[cfg(feature = "tmux_2_4")]
    {
        v.push("command-alias[0] \"split-pane=split-window\"");
        v.push("command-alias[1] \"splitp=split-window\"");
        v.push("command-alias[2] \"server-info=show-messages -JT\"");
        v.push("command-alias[3] \"info=show-messages -JT\"");
        v.push("command-alias[4] \"choose-window=choose-tree -w\"");
        v.push("command-alias[5] \"choose-session=choose-tree -s\"");
    }
    #[cfg(feature = "tmux_2_1")]
    v.push("default-terminal \"screen-256color\"");
    #[cfg(feature = "tmux_3_2")]
    v.push("copy-command ''");
    #[cfg(feature = "tmux_3_2")]
    v.push("editor /usr/bin/vi");
    #[cfg(feature = "tmux_1_2")]
    v.push("escape-time 500");
    #[cfg(feature = "tmux_2_7")]
    v.push("exit-empty on");
    #[cfg(feature = "tmux_1_4")]
    v.push("exit-unattached off");
    #[cfg(feature = "tmux_3_2")]
    v.push("extended-keys off");
    #[cfg(feature = "tmux_1_9")]
    v.push("focus-events off");
    #[cfg(feature = "tmux_2_1")]
    v.push("history-file \"\"");
    #[cfg(feature = "tmux_2_0")]
    v.push("message-limit 100");
    #[cfg(feature = "tmux_3_3")]
    v.push("prompt-history-limit 100");
    #[cfg(feature = "tmux_1_5")]
    v.push("set-clipboard external");
    #[cfg(feature = "tmux_3_2")]
    {
        v.push("terminal-features[0] \"xterm*:clipboard:ccolour:cstyle:focus:title\"");
        v.push("terminal-features[1] \"screen*:title\"");
    }
    #[cfg(feature = "tmux_2_0")]
    {
        v.push("terminal-overrides[0] \"xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q\"");
        v.push("terminal-overrides[1] \"screen*:XT\"");
    }
    //#[cfg(feature = "tmux_3_0")]
    //pub user_keys: Option<Vec<String>>,
    //#[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    //pub quiet: Option<Switch>,
    //#[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    //pub detach_on_destroy: Option<Switch>,
    // `@USER_OPTION`
    //pub user_options: Option<HashMap<String, String>>
    let options_raw = v.join("\n");

    let options: ServerOptions = options_raw.parse().unwrap();

    //dbg!(&options);

    let origin = ServerOptions::new();
    #[cfg(feature = "tmux_3_1")]
    let origin = origin.backspace("");
    #[cfg(feature = "tmux_1_5")]
    let origin = origin.buffer_limit(50);
    #[cfg(feature = "tmux_2_4")]
    let origin = origin.command_alias(vec![
        "\"split-pane=split-window\"",
        "\"splitp=split-window\"",
        "\"server-info=show-messages -JT\"",
        "\"info=show-messages -JT\"",
        "\"choose-window=choose-tree -w\"",
        "\"choose-session=choose-tree -s\"",
    ]);

    #[cfg(feature = "tmux_2_1")]
    let origin = origin.default_terminal("\"screen-256color\"");
    #[cfg(feature = "tmux_3_2")]
    let origin = origin.copy_command("");
    #[cfg(feature = "tmux_1_2")]
    let origin = origin.escape_time(500);
    #[cfg(feature = "tmux_3_2")]
    let origin = origin.editor("");
    #[cfg(feature = "tmux_2_7")]
    let origin = origin.exit_empty(Switch::On);
    #[cfg(feature = "tmux_1_4")]
    let origin = origin.exit_unattached(Switch::Off);
    #[cfg(feature = "tmux_3_2")]
    let origin = origin.extended_keys("");
    #[cfg(feature = "tmux_1_9")]
    let origin = origin.focus_events(Switch::Off);
    #[cfg(feature = "tmux_2_1")]
    let origin = origin.history_file("\"\"");
    #[cfg(feature = "tmux_2_0")]
    let origin = origin.message_limit(100);
    #[cfg(feature = "tmux_3_3")]
    let origin = origin.prompt_history_limit("");
    #[cfg(feature = "tmux_1_5")]
    let origin = origin.set_clipboard(SetClipboard::External);
    #[cfg(feature = "tmux_3_2")]
    let origin = origin.terminal_features("");
    #[cfg(feature = "tmux_2_0")]
    let origin = origin.terminal_overrides(vec!["\"xterm*:XT:Ms=\\E]52;%p1%s;%p2%s\\007:Cs=\\E]12;%p1%s\\007:Cr=\\E]112\\007:Ss=\\E[%p1%d q:Se=\\E[2 q\"", "\"screen*:XT\""]);
    #[cfg(feature = "tmux_3_0")]
    let origin = origin.user_keys("");
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let origin = origin.quiet("");
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    let origin = origin.detach_on_destroy("");

    assert_eq!(origin, options);

    // `@USER_OPTION`
    //pub user_options: Option<HashMap<String, String>>
}
