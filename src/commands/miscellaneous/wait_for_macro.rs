// TODO: enum for arg
// FIXME: not multiple, only one choice
/// # Manual
///
/// tmux ^1.9:
/// ```text
/// wait-for [-L | -S | -U] channel
/// (alias: wait)
/// ```
///
/// tmux ^1.8:
/// ```text
/// wait-for -LSU channel
/// (alias: wait)
/// ```
#[macro_export]
macro_rules! wait_for {
    // `[-L]`
    (@cmd ($cmd:expr) -L, $($tail:tt)*) => {{
        $crate::wait_for!(@cmd ({
            $cmd.locked()
        }) $($tail)*)
    }};
    // `[-S]`
    (@cmd ($cmd:expr) -S, $($tail:tt)*) => {{
        $crate::wait_for!(@cmd ({
            $cmd.woken()
        }) $($tail)*)
    }};
    // `[-U]`
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::wait_for!(@cmd ({
            $cmd.unlocked()
        }) $($tail)*)
    }};
    // `channel`
    (@cmd ($cmd:expr) $channel:expr, $($tail:tt)*) => {{
        $crate::wait_for!(@cmd ({
            $cmd.channel($channel)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::WaitFor::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::wait_for!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::wait_for!(@cmd ({ $crate::WaitFor::new() }) $($tail)*,)
    }};
}

#[test]
fn wait_for_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^1.9:
    // ```text
    // wait-for [-L | -S | -U] channel
    // (alias: wait)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // wait-for -LSU channel
    // (alias: wait)
    // ```
    let wait_for = wait_for!();
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for!((wait_for), -L);
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for!((wait_for), -S);
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for!((wait_for), -U);
    #[cfg(feature = "tmux_1_8")]
    let wait_for = wait_for!((wait_for), "1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "wait-for";
    #[cfg(feature = "cmd_alias")]
    let cmd = "wait";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-L");
    #[cfg(feature = "tmux_1_8")]
    s.push("-S");
    #[cfg(feature = "tmux_1_8")]
    s.push("-U");
    #[cfg(feature = "tmux_1_8")]
    s.push("1");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let wait_for = wait_for.build().to_vec();

    assert_eq!(wait_for, s);
}
