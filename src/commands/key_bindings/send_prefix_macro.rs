/// # Manual
///
/// tmux ^1.6
/// ```text
/// send-prefix [-2] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// send-prefix [-t target-pane]
/// ```
#[macro_export]
macro_rules! send_prefix {
    // `[-2]`
    (@cmd ($cmd:expr) -2, $($tail:tt)*) => {{
        $crate::send_prefix!(@cmd ({
            $cmd.secondary()
        }) $($tail)*)
    }};
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::send_prefix!(@cmd ({
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
        $crate::SendPrefix::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::send_prefix!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::send_prefix!(@cmd ({ $crate::SendPrefix::new() }) $($tail)*,)
    }};
}

#[test]
fn send_prefix_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.6
    // ```text
    // send-prefix [-2] [-t target-pane]
    // ```
    //
    // tmux ^0.8:
    // ```text
    // send-prefix [-t target-pane]
    // ```
    let send_prefix = send_prefix!();
    #[cfg(feature = "tmux_1_6")]
    let send_prefix = send_prefix!((send_prefix), -2);
    #[cfg(feature = "tmux_0_8")]
    let send_prefix = send_prefix!((send_prefix), -t "1");

    let cmd = "send-prefix";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_6")]
    s.push("-2");
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "1"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let send_prefix = send_prefix.build().to_vec();

    assert_eq!(send_prefix, s);
}
