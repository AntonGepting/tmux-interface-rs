/// # Manual
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
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before!((confirm_before), -p "1");
    #[cfg(feature = "tmux_0_9")]
    let confirm_before = confirm_before!((confirm_before), -t "2");
    #[cfg(feature = "tmux_0_9")]
    let confirm_before = confirm_before!((confirm_before), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "confirm-before";
    #[cfg(feature = "cmd_alias")]
    let cmd = "confirm";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-p", "1"]);
    #[cfg(feature = "tmux_0_9")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_0_9")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let confirm_before = confirm_before.build().to_vec();

    assert_eq!(confirm_before, s);
}
