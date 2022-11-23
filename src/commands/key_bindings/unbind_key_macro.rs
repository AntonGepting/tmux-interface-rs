/// # Manual
///
/// tmux ^3.2:
/// ```text
/// unbind-key [-anq] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.4:
/// ```text
/// unbind-key [-an] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.1:
/// ```text
/// unbind-key [-acn] [-t mode-table] [-T key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^2.0:
/// ```text
/// unbind-key [-acn] [-t mode-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^1.4:
/// ```text
/// unbind-key [-acn] [-t key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^1.0:
/// ```text
/// unbind-key [-cn] [-t key-table] key
/// (alias: unbind)
/// ```
///
/// tmux ^0.8:
/// ```text
/// unbind-key key
/// (alias: unbind)
/// ```
// FIXME: -t -T flags
#[macro_export]
macro_rules! unbind_key {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-c]`
    (@cmd ($cmd:expr) -c, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.command_mode()
        }) $($tail)*)
    }};
    // `[-n]`
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.root()
        }) $($tail)*)
    }};
    // `[-q]`
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};
    // `[-t mode-key]`
    (@cmd ($cmd:expr) -t $mode_key:expr, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.mode_key($mode_key)
        }) $($tail)*)
    }};
    // `[-t key-table]`
    // `[-T key-table]`
    (@cmd ($cmd:expr) -T $key_table:expr, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.key_table($key_table)
        }) $($tail)*)
    }};
    // `key`
    (@cmd ($cmd:expr) $key:expr, $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({
            $cmd.key($key)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::UnbindKey::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::unbind_key!(@cmd ({ $crate::UnbindKey::new() }) $($tail)*,)
    }};
}

#[test]
fn unbind_key_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // unbind-key [-anq] [-T key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // unbind-key [-an] [-T key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // unbind-key [-acn] [-t mode-table] [-T key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^2.0:
    // ```text
    // unbind-key [-acn] [-t mode-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^1.4:
    // ```text
    // unbind-key [-acn] [-t key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // unbind-key [-cn] [-t key-table] key
    // (alias: unbind)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // unbind-key key
    // (alias: unbind)
    // ```
    let unbind_key = unbind_key!();
    #[cfg(feature = "tmux_1_4")]
    let unbind_key = unbind_key!((unbind_key), -a);
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    let unbind_key = unbind_key!((unbind_key), -c);
    #[cfg(feature = "tmux_1_0")]
    let unbind_key = unbind_key!((unbind_key), -n);
    #[cfg(feature = "tmux_3_2")]
    let unbind_key = unbind_key!((unbind_key), -q);
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    let unbind_key = unbind_key!((unbind_key), -t "1");
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //let unbind_key = unbind_key!((unbind_key), -t "2");
    #[cfg(feature = "tmux_1_0")]
    let unbind_key = unbind_key!((unbind_key), -T "2");
    #[cfg(feature = "tmux_0_8")]
    let unbind_key = unbind_key!((unbind_key), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "unbind-key";
    #[cfg(feature = "cmd_alias")]
    let cmd = "unbind";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_4")]
    s.push("-a");
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    s.push("-c");
    #[cfg(feature = "tmux_1_0")]
    s.push("-n");
    #[cfg(feature = "tmux_3_2")]
    s.push("-q");
    #[cfg(all(feature = "tmux_2_0", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "1"]);
    //#[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_4")))]
    //s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_4")]
    s.extend_from_slice(&["-T", "2"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let unbind_key = unbind_key.build().to_vec();

    assert_eq!(unbind_key, s);
}
