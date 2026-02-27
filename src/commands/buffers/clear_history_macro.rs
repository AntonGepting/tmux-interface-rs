// auto-generated file
//

/// Remove and free the history for the specified pane.
///
/// # Manual
///
/// tmux ^3.4:
/// ```text
/// clear-history [-H] [-t target-pane]
/// (alias: clearhist)
/// ```
///
/// tmux ^1.5:
/// ```text
/// clear-history [-t target-pane]
/// (alias: clearhist)
/// ```
#[macro_export]
macro_rules! clear_history {
    // `[-H]`
    (@cmd ($cmd:expr) -H, $($tail:tt)*) => {{
        $crate::clear_history!(@cmd ({
            $cmd.no_hyperlinks()
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::clear_history!(@cmd ({
            $cmd.target_pane($target_pane)
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
    use std::borrow::Cow;

    // Remove and free the history for the specified pane.
    //
    // # Manual
    //
    // tmux ^3.4:
    // ```text
    // clear-history [-H] [-t target-pane]
    // (alias: clearhist)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // clear-history [-t target-pane]
    // (alias: clearhist)
    // ```

    let clear_history = clear_history!();
    #[cfg(feature = "tmux_3_4")]
    let clear_history = clear_history!((clear_history), -H);
    #[cfg(feature = "tmux_1_5")]
    let clear_history = clear_history!((clear_history), -t "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "clear-history";
    #[cfg(feature = "cmd_alias")]
    let cmd = "clearhist";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_4")]
    s.push("-H");
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let clear_history = clear_history.build().to_vec();

    assert_eq!(clear_history, s);
}
