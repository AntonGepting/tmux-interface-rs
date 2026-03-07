// auto-generated file
//

/// Display a menu on target-client
///
/// # Manual
///
/// tmux >=3.5:
/// ```text
/// display-menu [-OM] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
///
/// tmux >=3.4:
/// ```text
/// display-menu [-O] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command …
/// alias: menu
/// ```
///
/// tmux >=3.2:
/// ```text
/// display-menu [-O] [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
///
/// tmux >=3.0:
/// ```text
/// display-menu [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
/// alias: menu
/// ```
#[macro_export]
macro_rules! display_menu {
    // `[-O]`
    (@cmd ($cmd:expr) -O, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.not_close()
        }) $($tail)*)
    }};

    // `[-M]`
    (@cmd ($cmd:expr) -M, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.mouse_in_menu()
        }) $($tail)*)
    }};

    // `[-c target-client]`
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};

    // `[-T title]`
    (@cmd ($cmd:expr) -T $title:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.title($title)
        }) $($tail)*)
    }};

    // `[-x position]`
    (@cmd ($cmd:expr) -x $position:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.x($position)
        }) $($tail)*)
    }};

    // `[-y position]`
    (@cmd ($cmd:expr) -y $position:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.y($position)
        }) $($tail)*)
    }};

    // `[-H selected-style]`
    (@cmd ($cmd:expr) -H $selected_style:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.selected_style($selected_style)
        }) $($tail)*)
    }};

    // `[-s style]`
    (@cmd ($cmd:expr) -s $style:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.style($style)
        }) $($tail)*)
    }};

    // `[-S border-style]`
    (@cmd ($cmd:expr) -S $border_style:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.border_style($border_style)
        }) $($tail)*)
    }};

    // `[name]`
    (@cmd ($cmd:expr) $name:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.name($name)
        }) $($tail)*)
    }};

    // FIXME: no difference between key and command for macro
    // `[key]`
    (@cmd ($cmd:expr) --key $key:expr, $($tail:tt)*) => {{
        $crate::display_menu!(@cmd ({
            $cmd.key($key)
        }) $($tail)*)
    }};

    // `[command]`
    (@cmd ($cmd:expr) --command $command:expr, $($tail:tt)*) => {{
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
    use std::borrow::Cow;

    // Display a menu on target-client
    //
    // # Manual
    //
    // tmux >=3.5:
    // ```text
    // display-menu [-OM] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
    // alias: menu
    // ```
    //
    // tmux >=3.4:
    // ```text
    // display-menu [-O] [-b border-lines] [-c target-client] [-C starting-choice] [-H selected-style] [-s style] [-S border-style] [-t target-pane] [-T title] [-x position] [-y position] name key command …
    // alias: menu
    // ```
    //
    // tmux >=3.2:
    // ```text
    // display-menu [-O] [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
    // alias: menu
    // ```
    //
    // tmux >=3.0:
    // ```text
    // display-menu [-c target-client] [-t target-pane] [-T title] [-x position] [-y position] name key command ...
    // alias: menu
    // ```

    let display_menu = display_menu!();
    #[cfg(feature = "tmux_3_2")]
    let display_menu = display_menu!((display_menu), -O);
    #[cfg(feature = "tmux_3_5")]
    let display_menu = display_menu!((display_menu), -M);
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -c "1");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -t "2");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -T "3");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -x 4);
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), -y 5);
    #[cfg(feature = "tmux_3_4")]
    let display_menu = display_menu!((display_menu), -H "6");
    #[cfg(feature = "tmux_3_4")]
    let display_menu = display_menu!((display_menu), -s "7");
    #[cfg(feature = "tmux_3_4")]
    let display_menu = display_menu!((display_menu), -S "8");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), "9");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), --key "10");
    #[cfg(feature = "tmux_3_0")]
    let display_menu = display_menu!((display_menu), --command "11");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-menu";
    #[cfg(feature = "cmd_alias")]
    let cmd = "menu";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-O");
    #[cfg(feature = "tmux_3_5")]
    s.push("-M");
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
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-H", "6"]);
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-s", "7"]);
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-S", "8"]);
    #[cfg(feature = "tmux_3_0")]
    s.push("9");
    #[cfg(feature = "tmux_3_0")]
    s.push("10");
    #[cfg(feature = "tmux_3_0")]
    s.push("11");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let display_menu = display_menu.build().to_vec();

    assert_eq!(display_menu, s);
}
