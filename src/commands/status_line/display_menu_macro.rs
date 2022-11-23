/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// display-menu [-O] [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
///
/// tmux ^3.0:
/// ```text
/// display-menu [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
#[macro_export]
macro_rules! display_menu {
    // `[-O]` - the menu does not close when the mouse button is released without an item selected
    (@cmd ($cmd:expr) -O, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.not_close()
        }) $($tail)*)
    }};


    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.keep_zoomed()
        }) $($tail)*)
    }};
    // `[-c target-client]` - target-client
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-T title]` - title
    (@cmd ($cmd:expr) -T $title:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.title($title)
        }) $($tail)*)
    }};
    // `[-x position]` - x position of the menu
    (@cmd ($cmd:expr) -x $position:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.x($position)
        }) $($tail)*)
    }};
    // `[-y position]` - y position of the menu
    (@cmd ($cmd:expr) -y $position:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.y($position)
        }) $($tail)*)
    }};
    // `name`
    (@cmd ($cmd:expr) $name:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.name($name)
        }) $($tail)*)
    }};
    // `key`
    (@cmd ($cmd:expr) $key:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.key($key)
        }) $($tail)*)
    }};
    // `command`
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::DisplayMenu::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({ $crate::DisplayMenu::new() }) $($tail)*,)
    }};
}

#[test]
fn display_menu_macro() {
    use crate::{DisplayMenu, TargetPane};
    use std::borrow::Cow;

    // Structure for displaying a menu on target-client
    //
    // # Manual
    //
    // tmux ^3.0:
    // ```text
    // display-menu [-c target-client] [-t target-pane] [-T title]
    // [-x position] [-y position] name key command ...
    // ```
    let target_pane = TargetPane::Raw("2").to_string();

    let display_menu = display_menu!();
    #[cfg(feature = "tmux_3_2")]
    let display_menu = display_menu!((display_menu), -O);
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -c "1");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -t & target_pane);
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -T "3");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -x 4);
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -y 5);
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), "6");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), "7");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), "8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-menu";
    #[cfg(feature = "cmd_alias")]
    let cmd = "menu";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-O");
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-T", "3"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-x", "4"]);
    #[cfg(feature = "tmux_3_0")]
    s.extend_from_slice(&["-y", "5"]);
    #[cfg(feature = "tmux_3_0")]
    s.push("6");
    #[cfg(feature = "tmux_3_0")]
    s.push("7");
    #[cfg(feature = "tmux_3_0")]
    s.push("8");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_menu = display_menu.build().to_vec();

    assert_eq!(display_menu, s);
}
