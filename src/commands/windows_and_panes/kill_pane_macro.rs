// auto-generated file
//

/// Destroy the given pane
///
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// kill-pane [-a] [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux >=0.8:
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

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::kill_pane!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_1_5")]
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
    use std::borrow::Cow;

    // Destroy the given pane
    //
    // # Manual
    //
    // tmux >=1.5:
    // ```text
    // kill-pane [-a] [-t target-pane]
    // (alias: killp)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // kill-pane [-p pane-index] [-t target-window]
    // (alias: killp)
    // ```

    let kill_pane = kill_pane!();
    #[cfg(feature = "tmux_1_5")]
    let kill_pane = kill_pane!((kill_pane), -a);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let kill_pane = kill_pane!((kill_pane), -t "1");
    #[cfg(feature = "tmux_1_5")]
    let kill_pane = kill_pane!((kill_pane), -t "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "kill-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "killp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let kill_pane = kill_pane.build().to_vec();

    assert_eq!(kill_pane, s);
}
