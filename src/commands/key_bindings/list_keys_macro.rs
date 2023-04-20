/// # Manual
///
/// tmux ^3.1:
/// ```text
/// list-keys [-1aN] [-P prefix-string -T key-table]
/// (alias: lsk)
/// ```
///
/// tmux ^2.4:
/// ```text
/// list-keys [-T key-table]
/// (alias: lsk)
/// ```
///
/// tmux ^2.1:
/// ```text
/// list-keys [-t mode-table] [-T key-table]
/// (alias: lsk)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-keys [-t key-table]
/// (alias: lsk)
/// ```
// FIXME: -T key-table, -t key-table
#[macro_export]
macro_rules! list_keys {
    // `[-1]`
    (@cmd ($cmd:expr) -1, $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({
            $cmd.first()
        }) $($tail)*)
    }};
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({
            $cmd.command()
        }) $($tail)*)
    }};
    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({
            $cmd.with_notes()
        }) $($tail)*)
    }};
    // `[-P prefix-string]`
    (@cmd ($cmd:expr) -P $prefix_string:expr, $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({
            $cmd.prefix_string($prefix_string)
        }) $($tail)*)
    }};
    // `[-t mode-table]`
    // `[-t key-table]`
    (@cmd ($cmd:expr) -t $table:expr, $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({
            #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
            {
                $cmd.mode_table($table)
            }
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
            {
                $cmd.key_table($table)
            }
        }) $($tail)*)
    }};
    // `[-T key-table]`
    (@cmd ($cmd:expr) -T $key_table:expr, $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({
            $cmd.key_table($key_table)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListKeys::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_keys!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_keys!(@cmd ({ $crate::ListKeys::new() }) $($tail)*,)
    }};
}

#[test]
fn list_keys_macro() {
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.1:
    // ```text
    // list-keys [-1aN] [-P prefix-string -T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^2.4:
    // ```text
    // list-keys [-T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^2.1:
    // ```text
    // list-keys [-t mode-table] [-T key-table]
    // (alias: lsk)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-keys [-t key-table]
    // (alias: lsk)
    // ```
    let list_keys = list_keys!();
    #[cfg(feature = "tmux_2_4")]
    let list_keys = list_keys!((list_keys), -1);
    #[cfg(feature = "tmux_2_4")]
    let list_keys = list_keys!((list_keys), -a);
    #[cfg(feature = "tmux_2_4")]
    let list_keys = list_keys!((list_keys), -N);
    #[cfg(feature = "tmux_3_1")]
    let list_keys = list_keys!((list_keys), -P "1");
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    let list_keys = list_keys!((list_keys), -t "2");
    #[cfg(feature = "tmux_2_1")]
    let list_keys = list_keys!((list_keys), -T "3");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    let list_keys = list_keys!((list_keys), -t "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-keys";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsk";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_4")]
    s.push("-1");
    #[cfg(feature = "tmux_2_4")]
    s.push("-a");
    #[cfg(feature = "tmux_2_4")]
    s.push("-N");
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-P", "1"]);
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_2_4")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_2_1")]
    s.extend_from_slice(&["-T", "3"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_1")))]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_keys = list_keys.build().to_vec();

    assert_eq!(list_keys, s);
}
