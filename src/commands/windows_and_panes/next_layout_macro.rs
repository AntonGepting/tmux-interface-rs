/// Move a window to the next layout and rearrange the panes to fit
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// next-layout [-t target-window]
/// (alias: nextl)
/// ```
#[macro_export]
macro_rules! next_layout {
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::next_layout!(@cmd ({
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
        $crate::NextLayout::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::next_layout!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::next_layout!(@cmd ({ $crate::NextLayout::new() }) $($tail)*,)
    }};
}

#[test]
fn next_layout_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Move a window to the next layout and rearrange the panes to fit
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // next-layout [-t target-window]
    // (alias: nextl)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let next_layout = next_layout!();
    #[cfg(feature = "tmux_0_8")]
    let next_layout = next_layout!((next_layout), -t & target_window);

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "next-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "nextl";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let next_layout = next_layout.build().to_vec();

    assert_eq!(next_layout, s);
}
