/// Destroy the given pane
///
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// kill-pane [-a] [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// kill-pane [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// kill-pane [-p pane-index] [-t target-window]
/// (alias: killp)
/// ```
#[macro_export]
macro_rules! kill_pane {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};
    // `[-p pane-index]`
    (@cmd ($cmd:expr) -p $pane_index:expr, $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({
            $cmd.pane_index($pane_index)
        }) $($tail)*)
    }};
    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $shell_command:expr, $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({
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
        $crate::KillPane::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({ $crate::KillPane::new() }) $($tail)*,)
    }};
}

#[test]
fn kill_pane_macro() {
    use crate::TargetPane;
    use std::borrow::Cow;

    // Destroy the given pane
    //
    // # Manual
    //
    // tmux ^1.1:
    // ```text
    // kill-pane [-a] [-t target-pane]
    // (alias: killp)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // kill-pane [-t target-pane]
    // (alias: killp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // kill-pane [-p pane-index] [-t target-window]
    // (alias: killp)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let kill_pane = kill_pane!();
    #[cfg(feature = "tmux_1_1")]
    let kill_pane = kill_pane!((kill_pane), -a);
    #[cfg(feature = "tmux_1_0")]
    let kill_pane = kill_pane!((kill_pane), -t & target_pane);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    let kill_pane = kill_pane!((kill_pane), -p "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    let kill_pane = kill_pane!((kill_pane), -t "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_1")]
    s.push("-a");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let kill_pane = kill_pane.build().to_vec();

    assert_eq!(kill_pane, s);
}
