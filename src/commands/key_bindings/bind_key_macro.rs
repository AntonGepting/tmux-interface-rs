/// Structure binding key `key` to command
///
/// # Manual
///
/// tmux 3.1:
/// ```text
/// bind-key [-nr] [-N note] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.4:
/// ```text
/// bind-key [-nr] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.3:
/// ```text
/// bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// bind-key [-cnr] [-t mode-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// bind-key [-cnr] [-t key-table] key command [arguments]
/// (alias: bind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// bind-key [-r] key command [arguments]
/// (alias: bind)
/// ```
// FIXME three parameters
// FIXME: -c flag support
#[macro_export]
macro_rules! bind_key {
    // `[-n]` - an alias for -T root
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.root()
        }) $($tail)*)
    }};
    // `[-r]` - this key may repeat
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.repeat()
        }) $($tail)*)
    }};
    // `[-N note]` - attaches note to the key
    (@cmd ($cmd:expr) -N $note:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.note($note)
        }) $($tail)*)
    }};
    // `[-T key-table]` - key-table
    (@cmd ($cmd:expr) -T $key_table:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.key_table($key_table)
        }) $($tail)*)
    }};
    // `[arguments]` - arguments
    (@cmd ($cmd:expr) $arguments:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.arguments($arguments)
        }) $($tail)*)
    }};
    // `key`
    (@cmd ($cmd:expr) $key:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
            $cmd.key($key)
        }) $($tail)*)
    }};
    // `command`
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::bind_key!(@cmd ({
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

    // Structure binding key `key` to command
    //
    // # Manual
    //
    // tmux 3.1:
    // ```text
    // bind-key [-nr] [-N note] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // bind-key [-nr] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // bind-key [-cnr] [-R repeat-count] [-t mode-table] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // bind-key [-cnr] [-t mode-table] [-T key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // bind-key [-cnr] [-t mode-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // bind-key [-cnr] [-t key-table] key command [arguments]
    // (alias: bind)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // bind-key [-r] key command [arguments]
    // (alias: bind)
    // ```

    let bind_key = bind_key!();
    #[cfg(feature = "tmux_1_0")]
    let bind_key = bind_key!((bind_key), -n);
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), -r);
    #[cfg(feature = "tmux_3_1")]
    let bind_key = bind_key!((bind_key), -N "1");
    #[cfg(feature = "tmux_2_1")]
    let bind_key = bind_key!((bind_key), -T "2");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), "3");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), "4");
    #[cfg(feature = "tmux_0_8")]
    let bind_key = bind_key!((bind_key), "5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "bind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "bind";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_0")]
    s.push("-n");
    #[cfg(feature = "tmux_0_8")]
    s.push("-r");
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-N", "1"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let bind_key = bind_key.build().to_vec();

    assert_eq!(bind_key, s);
}
