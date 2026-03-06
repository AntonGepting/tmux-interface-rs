// auto-generated file
//

/// Ask for confirmation before executing command
///
/// # Manual
///
/// tmux >=3.4:
/// ```text
/// confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
///
/// tmux >=3.3:
/// ```text
/// confirm-before [-b] [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
///
/// tmux >=1.5:
/// ```text
/// confirm-before [-p prompt] [-t target-client] command
/// (alias: confirm)
/// ```
#[macro_export]
macro_rules! confirm_before {
    // `[-b]`
    (@cmd ($cmd:expr) -b, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.background()
        }) $($tail)*)
    }};

    // `[-y]`
    (@cmd ($cmd:expr) -y, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.change_default()
        }) $($tail)*)
    }};

    // `[-c confirm-key]`
    (@cmd ($cmd:expr) -c $confirm_key:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.confirm_key($confirm_key)
        }) $($tail)*)
    }};

    // `[-p prompt]`
    (@cmd ($cmd:expr) -p $prompt:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.prompt($prompt)
        }) $($tail)*)
    }};

    // `[-t target-client]`
    (@cmd ($cmd:expr) -t $target_client:expr, $($tail:tt)*) => {{
        $crate::confirm_before!(@cmd ({
            $cmd.target_client($target_client)
        }) $($tail)*)
    }};

    // `[command]`
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

    // Ask for confirmation before executing command
    //
    // # Manual
    //
    // tmux >=3.4:
    // ```text
    // confirm-before [-by] [-c confirm-key] [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux >=3.3:
    // ```text
    // confirm-before [-b] [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // confirm-before [-p prompt] [-t target-client] command
    // (alias: confirm)
    // ```

    let confirm_before = confirm_before!();
    #[cfg(feature = "tmux_3_3")]
    let confirm_before = confirm_before!((confirm_before), -b);
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before!((confirm_before), -y);
    #[cfg(feature = "tmux_3_4")]
    let confirm_before = confirm_before!((confirm_before), -c "1");
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before!((confirm_before), -p "2");
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before!((confirm_before), -t "3");
    #[cfg(feature = "tmux_1_5")]
    let confirm_before = confirm_before!((confirm_before), "4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "confirm-before";
    #[cfg(feature = "cmd_alias")]
    let cmd = "confirm";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_3")]
    s.push("-b");
    #[cfg(feature = "tmux_3_4")]
    s.push("-y");
    #[cfg(feature = "tmux_3_4")]
    s.extend_from_slice(&["-c", "1"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-p", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let confirm_before = confirm_before.build().to_vec();

    assert_eq!(confirm_before, s);
}
