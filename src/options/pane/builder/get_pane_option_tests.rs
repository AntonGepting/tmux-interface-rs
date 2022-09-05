#[test]
fn get_pane_option() {
    use crate::options::get_pane_option::GetPaneOption;

    let cmd = "show -p";

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {}", cmd, "allow-rename");
        let get_option = GetPaneOption::allow_rename().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {}", cmd, "alternate-screen");
        let get_option = GetPaneOption::alternate_screen().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {}", cmd, "remain-on-exit");
        let get_option = GetPaneOption::remain_on_exit().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {}", cmd, "window-active-style");
        let get_option = GetPaneOption::window_active_style().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_0")]
    {
        let origin = format!("{} {}", cmd, "window-style");
        let get_option = GetPaneOption::window_style().to_string();
        assert_eq!(origin, get_option);
    }

    #[cfg(feature = "tmux_3_2")]
    {
        let origin = format!("{} {}", cmd, "synchronize-panes");
        let get_option = GetPaneOption::synchronize_panes().to_string();
        assert_eq!(origin, get_option);
    }

    {
        let origin = format!("{} {}", cmd, "@user-option-name");
        let get_option = GetPaneOption::user_option("user-option-name").to_string();
        assert_eq!(origin, get_option);
    }
}
