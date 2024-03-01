/// # Manual
///
/// tmux ^3.4:
/// ```text
/// confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
///
/// tmux ^1.5:
/// ```text
/// confirm-before [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
///
/// tmux ^0.9:
/// ```text
/// confirm-before [-t target-client] command
/// (alias: confirm)
/// ```
#[macro_export]
macro_rules! confirm_before {
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.background()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -y, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.change_default()
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -c $confirm_key:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.confirm_key($confirm_key)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -p $prompt:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.prompt($prompt)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ConfirmBefore::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({ $crate::ConfirmBefore::new() }) $($tail)*,)
    }};
}

#[test]
fn confirm_before_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.4:
    // ```text
    // confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // confirm-before [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux ^0.9:
    // ```text
    // confirm-before [-t target-client] command
    // (alias: confirm)
    // ```
    let confirm_before = confirm_before!();
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before!((confirm_before), -b);
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before!((confirm_before), -y);
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before!((confirm_before), -c "1");
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before!((confirm_before), -p "2");
    #[cfg(feature = "tmux_0_9")]
    let confirm_before = confirm_before!((confirm_before), -t "3");
    #[cfg(feature = "tmux_0_9")]
    let confirm_before = confirm_before!((confirm_before), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "confirm-before";
    #[cfg(feature = "cmd_alias")]
    let cmd = "confirm";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_4")]
    s.push("-b");
    #[cfg(feature = "tmux_3_4")]
    s.push("-y");
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_9")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let confirm_before = confirm_before.build().to_vec();

    assert_eq!(confirm_before, s);
}
