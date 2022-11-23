// XXX: better return type
/// List panes on the server
///
/// # Manual
///
/// tmux ^1.6:
/// ```text
/// list-panes [-as] [-F format] [-t target]
/// (alias: lsp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// list-panes [-as] [-t target]
/// (alias: lsp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-panes [-t target]
/// (alias: lsp)
/// ```
#[macro_export]
macro_rules! list_panes {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::list_panes!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-s]`
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::list_panes!(@cmd ({
            $cmd.session()
        }) $($tail)*)
    }};
    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::list_panes!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-t target]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::list_panes!(@cmd ({
            $cmd.target($target)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListPanes::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_panes!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_panes!(@cmd ({ $crate::ListPanes::new() }) $($tail)*,)
    }};
}

#[test]
fn list_panes_macro() {
    use std::borrow::Cow;

    // List panes on the server
    //
    // # Manual
    //
    // tmux ^1.6:
    // ```text
    // list-panes [-as] [-F format] [-t target]
    // (alias: lsp)
    // ```
    //
    // tmux ^1.5:
    // ```text
    // list-panes [-as] [-t target]
    // (alias: lsp)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-panes [-t target]
    // (alias: lsp)
    // ```
    let list_panes = list_panes!();
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes!((list_panes), -a);
    #[cfg(feature = "tmux_1_5")]
    let list_panes = list_panes!((list_panes), -s);
    #[cfg(feature = "tmux_1_6")]
    let list_panes = list_panes!((list_panes), -F "1");
    #[cfg(feature = "tmux_0_8")]
    let list_panes = list_panes!((list_panes), -t "2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-panes";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsp";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(feature = "tmux_1_5")]
    s.push("-s");
    #[cfg(feature = "tmux_1_6")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.extend_from_slice(&["-t", "2"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_panes = list_panes.build().to_vec();

    assert_eq!(list_panes, s);
}
