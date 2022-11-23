/// Display a visible indicator of each pane shown by `target-client`
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// display-panes [-bN] [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.9:
/// ```text
/// display-panes [-b] [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// display-panes [-d duration] [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^2.3:
/// ```text
/// display-panes [-t target-client] [template]
/// (alias: displayp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// display-panes [-t target-client]
/// (alias: displayp)
/// ```
#[macro_export]
macro_rules! display_panes {
    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::display_panes!(@cmd ({
            $cmd.not_block()
        }) $($tail)*)
    }};
    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::display_panes!(@cmd ({
            $cmd.ignore_keys()
        }) $($tail)*)
    }};
    // XXX: type?
    // `[-d duration]`
    (@cmd ($cmd:expr) -d $duration:expr, $($tail:tt)*) => {{
        $crate::display_panes!(@cmd ({
            $cmd.duration($duration)
        }) $($tail)*)
    }};
    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::display_panes!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    // `[template]`
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::display_panes!(@cmd ({
            $cmd.template($template)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::DisplayPanes::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::display_panes!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::display_panes!(@cmd ({ $crate::DisplayPanes::new() }) $($tail)*,)
    }};
}

#[test]
fn display_panes() {
    use std::borrow::Cow;

    // Display a visible indicator of each pane shown by `target-client`
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // display-panes [-bN] [-d duration] [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^2.9:
    // ```text
    // display-panes [-b] [-d duration] [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^2.6:
    // ```text
    // display-panes [-d duration] [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // display-panes [-t target-client] [template]
    // (alias: displayp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // display-panes [-t target-client]
    // (alias: displayp)
    // ```
    let display_panes = display_panes!();
    #[cfg(feature = "tmux_2_9")]
    let display_panes = display_panes!((display_panes), -b);
    #[cfg(feature = "tmux_3_2")]
    let display_panes = display_panes!((display_panes), -N);
    #[cfg(feature = "tmux_2_6")]
    let display_panes = display_panes!((display_panes), -d "1");
    #[cfg(feature = "tmux_1_0")]
    let display_panes = display_panes!((display_panes), -t "2");
    #[cfg(feature = "tmux_2_3")]
    let display_panes = display_panes!((display_panes), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "display-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "displayp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_9")]
    s.push("-b");
    #[cfg(feature = "tmux_3_2")]
    s.push("-N");
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_3")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let display_panes = display_panes.build().to_vec();

    assert_eq!(display_panes, s);
}
