//#[test]
//fn session_options_default() {
//use crate::SessionOptions;

//let session_options = SessionOptions {
//..Default::default()
//};
//dbg!(&session_options);
//}

#[test]
fn session_options_parse() {
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

//#[test]
//fn get() {
//use crate::SessionOptions;

//let session_options = SessionOptions::get_all().unwrap();

//dbg!(session_options);
//}
