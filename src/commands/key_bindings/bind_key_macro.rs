// auto-generated file
//

/// Bind key `key` to command
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// bind-key [-nr] [-N note] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.4:
/// ```text
/// bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.3:
/// ```text
/// bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.1:
/// ```text
/// bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=2.0:
/// ```text
/// bind-key [-cnr] [-t mode-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=1.5:
/// ```text
/// bind-key [-cnr] [-t key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux >=0.8:
/// ```text
/// bind-key [-r] key command [arguments]
/// (alias: bind)
/// ```
#[macro_export]
macro_rules! bind_key {
    // `[-c]`
    (@cmd ($cmd:expr) -c, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.command_mode()
        }) $($tail)*)
    }};

    // `[-n]`
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.root()
        }) $($tail)*)
    }};

    // `[-r]`
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.repeat()
        }) $($tail)*)
    }};

    // `[-N note]`
    (@cmd ($cmd:expr) -N $note:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.note($note)
        }) $($tail)*)
    }};

    // `[-R repeat-count]`
    (@cmd ($cmd:expr) -R $repeat_count:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.repeat_count($repeat_count)
        }) $($tail)*)
    }};

    // `[-t key-table]`
    // `[-t mode-table]`
    (@cmd ($cmd:expr) -t $table:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
            {
                $cmd.key_table($table)
            }
            #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
            {
                $cmd.mode_table($table)
            }
        }) $($tail)*)
    }};

    // `[-T key-table]`
    (@cmd ($cmd:expr) -T $key_table:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.key_table($key_table)
        }) $($tail)*)
    }};

    // `[key]`
    (@cmd ($cmd:expr) $key:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.key($key)
        }) $($tail)*)
    }};

    // FIXME: no difference between key, command and arguments for macro
    // `[command]`
    (@cmd ($cmd:expr) --command $command:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};

    // FIXME: no difference between key, command and arguments for macro
    // `[arguments]`
    (@cmd ($cmd:expr) --argument $arguments:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.arguments($arguments)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::BindKey::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({ $crate::BindKey::new() }) $($tail)*,)
    }};
}

#[test]
fn bind_key_macro() {
    use std::borrow::Cow;

    // Bind key `key` to command
    //
    // # Manual
    //
    // tmux >=3.1:
    // ```text
    // bind-key [-nr] [-N note] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux >=2.4:
    // ```text
    // bind-key [-nr] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux >=2.3:
    // ```text
    // bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux >=2.0:
    // ```text
    // bind-key [-cnr] [-t mode-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // bind-key [-cnr] [-t key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // bind-key [-r] key command [arguments]
    // (alias: bind)
    // ```

    let bind_key = bind_key!();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    let bind_key = bind_key!((bind_key), -c);
    #[cfg(feature = "tmux_1_5")]
    let bind_key = bind_key!((bind_key), -n);
    #[cfg(feature = "tmux_1_5")]
    let bind_key = bind_key!((bind_key), -r);
    #[cfg(feature = "tmux_3_1")]
    let bind_key = bind_key!((bind_key), -N "1");
    #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
    let bind_key = bind_key!((bind_key), -R "2");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    let bind_key = bind_key!((bind_key), -t "3");
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    let bind_key = bind_key!((bind_key), -t "4");
    #[cfg(feature = "tmux_2_1")]
    let bind_key = bind_key!((bind_key), -T "5");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), "6");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), --command "7");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), --argument "8");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "bind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "bind";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_4")))]
    s.push("-c");
    #[cfg(feature = "tmux_1_5")]
    s.push("-n");
    #[cfg(feature = "tmux_1_5")]
    s.push("-r");
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-N", "1"]);
    #[cfg(all(feature = "tmux_2_3", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-R", "2"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_0")))]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "4"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-T", "5"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("6");
    #[cfg(feature = "tmux_0_8")]
    s.push("7");
    #[cfg(feature = "tmux_0_8")]
    s.push("8");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let bind_key = bind_key.build().to_vec();

    assert_eq!(bind_key, s);
}
