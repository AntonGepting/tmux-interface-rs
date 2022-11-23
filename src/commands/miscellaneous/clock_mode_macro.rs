/// # Manual
///
/// tmux ^1.0:
/// ```text
/// clock-mode [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// clock-mode [-t target-window]
/// ```
#[macro_export]
macro_rules! clock_mode {
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::clock_mode!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::clock_mode!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ClockMode::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::clock_mode!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::clock_mode!(@cmd ({ $crate::ClockMode::new() }) $($tail)*,)
    }};
}

#[test]
fn clock_mode_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.0:
    // ```text
    // clock-mode [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // clock-mode [-t target-window]
    // ```
    let clock_mode = clock_mode!();
    #[cfg(feature = "tmux_1_0")]
    let clock_mode = clock_mode!((clock_mode), -t "1");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    let clock_mode = clock_mode!((clock_mode), -t "2");

    let cmd = "clock-mode";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let clock_mode = clock_mode.build().to_vec();

    assert_eq!(clock_mode, s);
}
