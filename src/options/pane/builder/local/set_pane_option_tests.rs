#[test]
fn set_pane_option() {
    use crate::{RemainOnExit, SetPaneOption, SetPaneOptionTr, SetUserOption, Switch};

    let cmd = "set -p";
    let target = ":";
    let cmd = format!("{} -t {}", cmd, target);

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {} {}", cmd, "allow-rename", "off");
        let get_option = SetPaneOption::allow_rename(Some(target), Some(Switch::Off)).to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {} {}", cmd, "alternate-screen", "off");
        let get_option =
            SetPaneOption::alternate_screen(Some(target), Some(Switch::Off)).to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {} {}", cmd, "remain-on-exit", "off");
        let get_option =
            SetPaneOption::remain_on_exit(Some(target), Some(RemainOnExit::Off)).to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {} {}", cmd, "window-active-style", "1");
        let get_option =
            SetPaneOption::window_active_style(Some(target), Some("1".to_string())).to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {} {}", cmd, "window-style", "2");
        let get_option =
            SetPaneOption::window_style(Some(target), Some("2".to_string())).to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {} {}", cmd, "synchronize-panes", "off");
        let get_option =
            SetPaneOption::synchronize_panes(Some(target), Some(Switch::Off)).to_string();
        assert_eq!(origin, get_option);
    }

    {
        let origin = format!("{} {} {}", cmd, "@user-option-name", "value");
        let get_option = SetPaneOption::user_option_ext(
            Some(target),
            "user-option-name",
            Some("value".to_string()),
        )
        .to_string();
        assert_eq!(origin, get_option);
    }
}
