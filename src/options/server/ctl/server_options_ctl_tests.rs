#[test]
fn server_options_ctl() {
    use crate::{ServerOptionsCtl, Tmux};

    // let server_options_ctl = ServerOptionsCtl::default();
    let server_options_ctl = ServerOptionsCtl::new(&|cmd| Tmux::with_command(cmd).output());

    let value = server_options_ctl.get_all().unwrap();

    dbg!(&value);
}

#[test]
fn server_option() {
    use crate::ServerOptionsCtl;

    let server_options_ctl = ServerOptionsCtl::default();
    //let server_option_ctl = ServerOption::new(|cmd| Tmux::with_command(cmd).output());

    #[cfg(feature = "tmux_3_1")]
    let backspace = server_options_ctl.get_backspace().unwrap();
    #[cfg(feature = "tmux_1_5")]
    let buffer_limit = server_options_ctl.get_buffer_limit().unwrap();
    #[cfg(feature = "tmux_2_4")]
    let command_alias = server_options_ctl.get_command_alias().unwrap();
    #[cfg(feature = "tmux_3_2")]
    let copy_command = server_options_ctl.get_copy_command().unwrap();
    #[cfg(feature = "tmux_2_1")]
    let default_terminal = server_options_ctl.get_default_terminal().unwrap();
    #[cfg(feature = "tmux_1_2")]
    let escape_time = server_options_ctl.get_escape_time().unwrap();
    #[cfg(feature = "tmux_3_2")]
    let editor = server_options_ctl.get_editor().unwrap();
    #[cfg(feature = "tmux_2_7")]
    let exit_empty = server_options_ctl.get_exit_empty().unwrap();
    #[cfg(feature = "tmux_1_4")]
    let exit_unattached = server_options_ctl.get_exit_unattached().unwrap();
    #[cfg(feature = "tmux_3_2")]
    let extended_keys = server_options_ctl.get_extended_keys().unwrap();
    #[cfg(feature = "tmux_1_9")]
    let focus_events = server_options_ctl.get_focus_events().unwrap();
    #[cfg(feature = "tmux_2_1")]
    let history_file = server_options_ctl.get_history_file().unwrap();
    #[cfg(feature = "tmux_2_0")]
    let message_limit = server_options_ctl.get_message_limit().unwrap();
    #[cfg(feature = "tmux_3_3")]
    let prompt_history_limit = server_option_ctl.get_prompt_history_limit().unwrap();
    #[cfg(feature = "tmux_1_5")]
    let set_clipboard = server_options_ctl.get_set_clipboard().unwrap();
    #[cfg(feature = "tmux_3_2")]
    let terminal_features = server_options_ctl.get_terminal_features().unwrap();
    #[cfg(feature = "tmux_2_0")]
    let terminal_overrides = server_options_ctl.get_terminal_overrides().unwrap();
    #[cfg(feature = "tmux_3_0")]
    let user_keys = server_options_ctl.get_user_keys().unwrap();
    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    let quiet = server_option_ctl.get_quiet().unwrap();
    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    let detach_on_destroy = server_option_ctl.get_detach_on_destroy().unwrap();

    // let buffer_limit = ServerOption::default().get_buffer_limit().unwrap();
    // let buffer_limit = ServerOption::default().get_command_alias().unwrap();
    // dbg!(buffer_limit);
    // let buffer_limit = ServerOption::default()
    //     .set_command_alias(Some(["asdf"]))
    //     .unwrap();
    // dbg!(buffer_limit);
    // let buffer_limit = ServerOption::default().get_command_alias().unwrap();
    // dbg!(buffer_limit);
    // server_options_ctl.set_buffer_limit(buffer_limit);
    // let buffer_limit = server_options_ctl.get_buffer_limit();
    // dbg!(buffer_limit);
}
