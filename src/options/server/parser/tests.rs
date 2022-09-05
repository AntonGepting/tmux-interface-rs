
//#[test]
//fn parse_server_option() {
//use crate::options::get_server_option::{SetServerOption, TmuxServerOptionOutput};
//use crate::Tmux;

//#[cfg(feature = "tmux_3_1")]
//{
//let origin = "C-?";
//let output = Tmux::new()
//.command(SetServerOption::backspace())
//.output()
//.unwrap();
//let value = TmuxServerOptionOutput::from(output).backspace().unwrap();
//assert_eq!(origin, value);
//}

//#[cfg(feature = "tmux_1_5")]
//{
//let origin = 50;
//let output = Tmux::new()
//.command(SetServerOption::buffer_limit())
//.output()
//.unwrap();
//let value = TmuxServerOptionOutput::from(output).buffer_limit().unwrap();
//assert_eq!(origin, value);
//}
//}

//#[test]
//fn get_server_option_c() {
//use crate::Tmux;

//let cmd = Tmux::new()
//.command(SetServerOption::get(BUFFER_LIMIT))
//.output()
//.unwrap();
//let cmd = Tmux::new()
//.command(SetServerOption::buffer_limit())
//.command(SetServerOption::set_clipboard())
//.output()
//.unwrap();
//dbg!(&cmd);
//let cmd = TmuxServerOptionOutput::from(cmd).buffer_limit();
//dbg!(&cmd);

//let cmd = Tmux::new()
//.command(SetServerOption::command_alias())
//.output()
//.unwrap();
//let cmd = TmuxServerOptionOutput::from(cmd).command_alias();
//dbg!(&cmd);

//let cmds = SetServerOption::command_alias(Some(vec!["asdf".to_string(), "a".to_string()]));
//dbg!(&cmds);
//}
