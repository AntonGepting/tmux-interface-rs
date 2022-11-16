/// Remove and free the history for the specified pane.
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// clear-history [-t target-pane]
/// (alias: clearhist)
/// ```
///
/// tmux ^0.9:
/// ```text
/// clear-history [-p pane-index] [-t target-window]
/// (alias: clearhist)
/// ```
#[macro_export]
macro_rules! clear_history {
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::clear_history!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::clear_history!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -p $pane_index:expr, $($tail:tt)*) => {{
        $crate::clear_history!(@cmd ({
            $cmd.pane_index($pane_index)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ClearHistory::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::clear_history!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::clear_history!(@cmd ({ $crate::ClearHistory::new() }) $($tail)*,)
    }};
}

#[test]
fn clear_history_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Remove and free the history for the specified pane.
    //
    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clear-history [-t target-pane]
    // (alias: clearhist)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // clear-history [-p pane-index] [-t target-window]
    // (alias: clearhist)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let clear_history = clear_history!();
    #[cfg(feature = "tmux_1_0")]
    let clear_history = clear_history!((clear_history), -t target_pane);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    let clear_history = clear_history!((clear_history), -p target_pane);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    let clear_history = clear_history!((clear_history), -p target_pane);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "clear-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "clearhist";

    let mut s = Vec::new();
    s.push(cmd);

    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let clear_history = clear_history.build().to_vec();

    assert_eq!(clear_history, s);
}
