// auto-generated file
//

/// Display a menu on target-client
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// display-popup [-BCEkN] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
/// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
/// [-y position] [shell-command]
/// ```
///
/// tmux >=3.3:
/// ```text
/// display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
/// [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
/// [-y position] [shell-command]
/// ```
///
/// tmux >=3.2:
/// ```text
/// display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
/// [-w width] [-x position] [-y position] [shell-command]
/// (alias: popup)
/// ```
#[macro_export]
macro_rules! display_popup {
    // `[-B]`
    (@cmd ($cmd:expr) -B, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.no_border()
        }) $($tail)*)
    }};

    // `[-C]`
    (@cmd ($cmd:expr) -C, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.close()
        }) $($tail)*)
    }};

    // `[-E]`
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.close_on_exit()
        }) $($tail)*)
    }};

    // `[-EE]`
    (@cmd ($cmd:expr) -EE, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.close_on_success()
        }) $($tail)*)
    }};

    // `[-k]`
    (@cmd ($cmd:expr) -k, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.any_key_dismiss()
        }) $($tail)*)
    }};

    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.disable_previous()
        }) $($tail)*)
    }};

    // `[-b border-lines]`
    (@cmd ($cmd:expr) -b $border_lines:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.border_lines($border_lines)
        }) $($tail)*)
    }};

    // `[-c target-client]`
    (@cmd ($cmd:expr) -c $target_client:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[-d start-directory]`
    (@cmd ($cmd:expr) -d $start_directory:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.start_directory($start_directory)
        }) $($tail)*)
    }};

    // `[-e environment]`
    (@cmd ($cmd:expr) -e $environment:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.environment($environment)
        }) $($tail)*)
    }};

    // `[-h height]`
    (@cmd ($cmd:expr) -h $height:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.height($height)
        }) $($tail)*)
    }};

    // `[-s style]`
    (@cmd ($cmd:expr) -s $style:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.style($style)
        }) $($tail)*)
    }};

    // `[-S border-style]`
    (@cmd ($cmd:expr) -S $border_style:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.border_style($border_style)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};

    // `[-T title]`
    (@cmd ($cmd:expr) -T $title:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.title($title)
        }) $($tail)*)
    }};

    // `[-w width]`
    (@cmd ($cmd:expr) -w $width:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.width($width)
        }) $($tail)*)
    }};

    // `[-x position]`
    (@cmd ($cmd:expr) -x $x:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.x($x)
        }) $($tail)*)
    }};

    // `[-y position]`
    (@cmd ($cmd:expr) -y $y:expr, $($tail:tt)*) => {{
        $crate::display_popup!(@cmd ({
            $cmd.y($y)
        }) $($tail)*)
    }};

    // `[shell-command]`
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
fn display_popup_macro() {
    use std::borrow::Cow;

    // Display a menu on target-client
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // display-popup [-BCEkN] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
    // [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
    // [-y position] [shell-command]
    // ```
    //
    // tmux >=3.3:
    // ```text
    // display-popup [-BCE] [-b border-lines] [-c target-client] [-d start-directory] [-e environment]
    // [-h height] [-s style] [-S border-style] [-t target-pane] [-T title] [-w width] [-x position]
    // [-y position] [shell-command]
    // ```
    //
    // tmux >=3.2:
    // ```text
    // display-popup [-CE] [-c target-client] [-d start-directory] [-h height] [-t target-pane]
    // [-w width] [-x position] [-y position] [shell-command]
    // (alias: popup)
    // ```

    let display_popup = display_popup!();
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -B);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -C);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -E);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -EE);
    #[cfg(feature = "tmux_3_6")]
    let display_popup = display_popup!((display_popup), -k);
    #[cfg(feature = "tmux_3_6")]
    let display_popup = display_popup!((display_popup), -N);
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -b "1");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -c "2");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -d "3");
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -e "4");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -h 5);
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -s "6");
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -S "7");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -t "8");
    #[cfg(feature = "tmux_3_3")]
    let display_popup = display_popup!((display_popup), -T "9");
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -w 10);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -x 11);
    #[cfg(feature = "tmux_3_2")]
    let display_popup = display_popup!((display_popup), -y 12);
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
    #[cfg(feature = "tmux_3_6")]
    s.push("-k");
    #[cfg(feature = "tmux_3_6")]
    s.push("-N");
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-b", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-c", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "3"]);
    #[cfg(feature = "tmux_3_3")]
    s.extend_from_slice(&["-e", "4"]);
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
