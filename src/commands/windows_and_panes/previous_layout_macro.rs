/// Move to the previous layout in the session
///
/// # Manual
///
/// tmux ^1.3:
/// ```text
/// previous-layout [-t target-window]
/// (alias: prevl)
/// ```
#[macro_export]
macro_rules! previous_layout {
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::previous_layout!(@cmd ({
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
        $crate::PreviousLayout::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::previous_layout!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::previous_layout!(@cmd ({ $crate::PreviousLayout::new() }) $($tail)*,)
    }};
}

#[test]
fn previous_layout_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Move to the previous layout in the session
    //
    // # Manual
    //
    // tmux ^1.3:
    // ```text
    // previous-layout [-t target-window]
    // (alias: prevl)
    // ```

    let target_window = TargetWindow::Raw("1").to_string();

    let previous_layout = previous_layout!();
    #[cfg(feature = "tmux_1_3")]
    let previous_layout = previous_layout!((previous_layout), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "previous-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "prevl";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_3")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let previous_layout = previous_layout.build().to_vec();

    assert_eq!(previous_layout, s);
}
