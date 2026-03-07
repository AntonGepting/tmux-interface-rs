// auto-generated file
//

/// Put a window into window choice mode
///
/// # Manual
///
/// tmux >=1.7 && <=2.5:
/// ```text
/// choose-window [-F format] [-t target-window] [template]
/// ```
///
/// tmux >=1.5:
/// ```text
/// choose-window [-t target-window] [template]
/// ```
///
/// tmux >=0.8:
/// ```text
/// choose-window [-t target-window]
/// ```
#[macro_export]
macro_rules! choose_window {
    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::choose_window!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::choose_window!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};

    // `[template]`
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::choose_window!(@cmd ({
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
        $crate::ChooseWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_window!(@cmd ({ $crate::ChooseWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn choose_window_macro() {
    use std::borrow::Cow;

    // Put a window into window choice mode
    //
    // # Manual
    //
    // tmux >=1.7 && <=2.5:
    // ```text
    // choose-window [-F format] [-t target-window] [template]
    // ```
    //
    // tmux >=1.5:
    // ```text
    // choose-window [-t target-window] [template]
    // ```
    //
    // tmux >=0.8:
    // ```text
    // choose-window [-t target-window]
    // ```

    let choose_window = choose_window!();
    #[cfg(feature = "tmux_1_7")]
    let choose_window = choose_window!((choose_window), -F "1");
    #[cfg(feature = "tmux_0_8")]
    let choose_window = choose_window!((choose_window), -t "2");
    #[cfg(feature = "tmux_1_5")]
    let choose_window = choose_window!((choose_window), "3");

    let cmd = "choose-window";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let choose_window = choose_window.build().to_vec();

    assert_eq!(choose_window, s);
}
