/// Rename the current window, or the window at target-window if specified, to new-name
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// rename-window [-t target-window] new-name
/// (alias: renamew)
/// ```
#[macro_export]
macro_rules! rename_window {
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::rename_window!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $new_name:expr, $($tail:tt)*) => {{
        $crate::rename_window!(@cmd ({
            $cmd.new_name($new_name)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::RenameWindow::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::rename_window!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::rename_window!(@cmd ({ $crate::RenameWindow::new() }) $($tail)*,)
    }};
}

#[test]
fn rename_window_macro() {
    use crate::TargetWindow;
    use std::borrow::Cow;

    // Rename the current window, or the window at target-window if specified, to new-name
    //
    // # Manual
    //
    // tmux ^0.8:
    // ```text
    // rename-window [-t target-window] new-name
    // (alias: renamew)
    // ```
    let target_window = TargetWindow::Raw("1").to_string();

    let rename_window = rename_window!();
    #[cfg(feature = "tmux_0_8")]
    let rename_window = rename_window!((rename_window), -t & target_window);
    #[cfg(feature = "tmux_0_8")]
    let rename_window = rename_window!((rename_window), "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "rename-window";
    #[cfg(feature = "cmd_alias")]
    let cmd = "renamew";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let rename_window = rename_window.build().to_vec();

    assert_eq!(rename_window, s);
}
