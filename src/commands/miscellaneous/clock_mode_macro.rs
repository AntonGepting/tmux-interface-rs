// auto-generated file
//

/// Display a large clock
///
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// clock-mode [-t target-pane]
/// ```
///
/// tmux >=0.8:
/// ```text
/// clock-mode [-t target-window]
/// ```
#[macro_export]
macro_rules! clock_mode {
    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::clock_mode!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_1_0")]
            {
                $cmd.target_pane($target)
            }
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

    // Display a large clock
    //
    // # Manual
    //
    // tmux >=1.5:
    // ```text
    // clock-mode [-t target-pane]
    // ```
    //
    // tmux >=0.8:
    // ```text
    // clock-mode [-t target-window]
    // ```

    let clock_mode = clock_mode!();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let clock_mode = clock_mode!((clock_mode), -t "1");
    #[cfg(feature = "tmux_1_5")]
    let clock_mode = clock_mode!((clock_mode), -t "2");

    let cmd = "clock-mode";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let clock_mode = clock_mode.build().to_vec();

    assert_eq!(clock_mode, s);
}
