/// Structure for displaying a menu on target-client
///
/// # Manual
///
/// tmux ^3.3:
/// ```text
/// display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
/// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
/// [-y position] [shell-command]
/// ```
///
/// tmux ^3.2:
/// ```text
/// display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
/// [-w width] [-x position] [-y position] [shell-command]
/// (alias: popup)
/// ```
#[macro_export]
macro_rules! display_popup {
    // `[-B]` - not surround the popup by a border
    (@cmd ($cmd:expr) -B, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.no_border()
        }) $($tail)*)
    }};
    // `[-C]` - closes any popup on the client
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.close()
        }) $($tail)*)
    }};
    // `[-E]` - closes the popup automatically when shell-command exits
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.close_on_exit()
        }) $($tail)*)
    }};
    // `[-EE]` - closes the popup only if shell-command exited with success
    (@cmd ($cmd:expr) -EE, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.close_on_success()
        }) $($tail)*)
    }};
    // `[-b border-lines]` - sets the type of border line for the popup.  When -B is specified, the
    // -b option is ignored.  See popup-border-lines for possible values for border-lines
    (@cmd ($cmd:expr) -b $border_lines:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.border_lines($border_lines)
        }) $($tail)*)
    }};
    // `[-c target-client]` - target-client
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[-d start-directory]` -
    (@cmd ($cmd:expr) -d $start_directory:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};
    // XXX: struct instead of tuple
    // `[-e environment]` - takes the form ‘VARIABLE=value’ and sets an environment variable for
    // the popup; it may be specified multiple times
    // (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        // $crate::display_popup!(@cmd ({
            // $cmd.environment($environment)
        // }) $($tail)*)
    // }};
    // `[-h height]` - height of the popup
    (@cmd ($cmd:expr) -h $height:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.height($height)
        }) $($tail)*)
    }};
    // TODO: not imlemented!
    // `[-s style]` - sets the style for the popup
    (@cmd ($cmd:expr) -s $style:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.style($style)
        }) $($tail)*)
    }};
    // TODO: not imlemented!
    // `[-S border-style]` - sets the style for the popup border
    (@cmd ($cmd:expr) -S $border_style:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.border_style($border_style)
        }) $($tail)*)
    }};
    // `[-t target-pane]` - target-pane
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-T title]` - is a format for the popup title
    (@cmd ($cmd:expr) -T $title:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.title($title)
        }) $($tail)*)
    }};
    // `[-w width]` - width of the popup
    (@cmd ($cmd:expr) -w $width:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.width($width)
        }) $($tail)*)
    }};
    // `[-x position]` - x position of the popup
    (@cmd ($cmd:expr) -x $position:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.x($position)
        }) $($tail)*)
    }};
    // `[-y position]` - y position of the popup
    (@cmd ($cmd:expr) -y $position:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.y($position)
        }) $($tail)*)
    }};
    // `[shell-command]` - shell-command
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.shell_command($shell_command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::DisplayPopup::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({ $crate::DisplayPopup::new() }) $($tail)*,)
    }};
}

#[test]
fn display_popup() {
    #[cfg(feature = "tmux_3_3")]
    use crate::commands::PopupBorderLinesType;
    use crate::commands::Size;
    use crate::TargetPane;
    use std::borrow::Cow;

    // Structure for displaying a menu on target-client
    //
    // # Manual
    //
    // tmux ^3.3:
    // ```text
    // display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
    // [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
    // [-y position] [shell-command]
    // ```
    //
    // tmux ^3.2:
    // ```text
    // display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
    // [-w width] [-x position] [-y position] [shell-command]
    // (alias: popup)
    // ```
    let target_pane = TargetPane::Raw("8").to_string();

    let display_popup = display_popup!();
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -B);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -C);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -E);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -EE);
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -b PopupBorderLinesType::NoBorder);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -c "1");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -d "2");
    // #[cfg(feature = "tmux_3_3")]
    // let display_popup = display_popup!((display_popup), -e "3", "4");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -h Size::Size(5));
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -s "6");
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -S "7");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -t & target_pane);
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -T "9");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -w Size::Size(10));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -x Size::Size(11));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -y Size::Size(12));
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), "13");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-popup";
    #[cfg(feature = "cmd_alias")]
    let cmd = "popup";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    s.push("-B");
    #[cfg(feature = "tmux_3_2")]
    s.push("-C");
    #[cfg(feature = "tmux_3_2")]
    s.push("-E");
    #[cfg(feature = "tmux_3_2")]
    s.push("-EE");
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-b", "none"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "2"]);
    // #[cfg(feature = "tmux_3_3")]
    // s.extend_from_slice(&["-e", "3=4"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-h", "5"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-s", "6"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-S", "7"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", "8"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-T", "9"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-w", "10"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-x", "11"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-y", "12"]);
    #[cfg(feature = "tmux_3_2")]
    s.push("13");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_popup = display_popup.build().to_vec();

    assert_eq!(display_popup, s);
}
