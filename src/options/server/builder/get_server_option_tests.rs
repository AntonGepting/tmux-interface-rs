//#[test]
//fn get_buffer_limit() {
//use crate::{GetServerOption, Tmux};

//#[cfg(feature = "tmux_1_5")]
//let get_option = Tmux::with_command(GetServerOption::buffer_limit()).output();
//}

#[test]
fn get_server_option() {
    use crate::options::get_server_option::GetServerOption;

    let cmd = "show -s -v";

    #[cfg(feature = "tmux_3_1")]
    {
        let origin = format!("{} {}", cmd, "backspace");
        let get_option = GetServerOption::backspace().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_1_5")]
    {
        let origin = format!("{} {}", cmd, "buffer-limit");
        let get_option = GetServerOption::buffer_limit().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_2_4")]
    {
        let origin = format!("{} {}", cmd, "command-alias");
        let get_option = GetServerOption::command_alias().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {}", cmd, "copy-command");
        let get_option = GetServerOption::copy_command().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {}", cmd, "default-terminal");
        let get_option = GetServerOption::default_terminal().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_1_2")]
    {
        let origin = format!("{} {}", cmd, "escape-time");
        let get_option = GetServerOption::escape_time().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {}", cmd, "editor");
        let get_option = GetServerOption::editor().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_2_7")]
    {
        let origin = format!("{} {}", cmd, "exit-empty");
        let get_option = GetServerOption::exit_empty().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_1_4")]
    {
        let origin = format!("{} {}", cmd, "exit-unattached");
        let get_option = GetServerOption::exit_unattached().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {}", cmd, "extended-keys");
        let get_option = GetServerOption::extended_keys().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_1_9")]
    {
        let origin = format!("{} {}", cmd, "focus-events");
        let get_option = GetServerOption::focus_events().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_2_1")]
    {
        let origin = format!("{} {}", cmd, "history-file");
        let get_option = GetServerOption::history_file().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_2_0")]
    {
        let origin = format!("{} {}", cmd, "message-limit");
        let get_option = GetServerOption::message_limit().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_3")]
    {
        let origin = format!("{} {}", cmd, "prompt-history-limit");
        let get_option = GetServerOption::prompt_history_limit().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_1_5")]
    {
        let origin = format!("{} {}", cmd, "set-clipboard");
        let get_option = GetServerOption::set_clipboard().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {}", cmd, "terminal-features");
        let get_option = GetServerOption::terminal_features().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_2_0")]
    {
        let origin = format!("{} {}", cmd, "terminal-overrides");
        let get_option = GetServerOption::terminal_overrides().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {}", cmd, "user-keys");
        let get_option = GetServerOption::user_keys().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_2_0")))]
    {
        let origin = format!("{} {}", cmd, "quiet");
        let get_option = GetServerOption::quiet().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(all(feature = "tmux_1_3", not(feature = "tmux_1_4")))]
    {
        let origin = format!("{} {}", cmd, "detach-on-destroy");
        let get_option = GetServerOption::detach_on_destroy().to_string();
        assert_eq!(origin, get_option);
    }

    {
        let origin = format!("{} {}", cmd, "@user_option_name");
        let get_option = GetServerOption::user_option("user_option_name").to_string();
        assert_eq!(origin, get_option);
    }
}

//#[test]
//fn parse_server_option() {
//use crate::options::get_server_option::{GetServerOption, TmuxServerOptionOutput};
//use crate::Tmux;

//#[cfg(feature = "tmux_3_1")]
//{
//let origin = "C-?";
//let output = Tmux::new()
//.command(GetServerOption::backspace())
//.output()
//.unwrap();
//let value = TmuxServerOptionOutput::from(output).backspace().unwrap();
//assert_eq!(origin, value);
//}

//#[cfg(feature = "tmux_1_5")]
//{
//let origin = 50;
//let output = Tmux::new()
//.command(GetServerOption::buffer_limit())
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
//.command(GetServerOption::get(BUFFER_LIMIT))
//.output()
//.unwrap();
//let cmd = Tmux::new()
//.command(GetServerOption::buffer_limit())
//.command(GetServerOption::set_clipboard())
//.output()
//.unwrap();
//dbg!(&cmd);
//let cmd = TmuxServerOptionOutput::from(cmd).buffer_limit();
//dbg!(&cmd);

//let cmd = Tmux::new()
//.command(GetServerOption::command_alias())
//.output()
//.unwrap();
//let cmd = TmuxServerOptionOutput::from(cmd).command_alias();
//dbg!(&cmd);

//let cmds = SetServerOption::command_alias(Some(vec!["asdf".to_string(), "a".to_string()]));
//dbg!(&cmds);
//}
