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
