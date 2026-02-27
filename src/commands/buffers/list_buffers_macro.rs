// auto-generated file
//

/// List the global buffers.
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// list-buffers [-F format] [-f filter]
/// (alias: lsb)
/// ```
///
/// tmux >=1.7:
/// ```text
/// list-buffers [-F format]
/// (alias: lsb)
/// ```
///
/// tmux >=1.5:
/// ```text
/// list-buffers
/// (alias: lsb)
/// ```
///
/// tmux >=0.8:
/// ```text
/// list-buffers [-t target-session]
/// (alias: lsb)
/// ```
#[macro_export]
macro_rules! list_buffers {
    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::list_buffers!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-f filter]`
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::list_buffers!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};

    // `[-t target_session]`
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::list_buffers!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListBuffers::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_buffers!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_buffers!(@cmd ({ $crate::ListBuffers::new() }) $($tail)*,)
    }};
}

#[test]
fn list_buffers_macro() {
    use std::borrow::Cow;

    // List the global buffers.
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // list-buffers [-F format] [-f filter]
    // (alias: lsb)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // list-buffers [-F format]
    // (alias: lsb)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // list-buffers
    // (alias: lsb)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // list-buffers [-t target-session]
    // (alias: lsb)
    // ```

    let list_buffers = list_buffers!();
    #[cfg(feature = "tmux_1_7")]
    let list_buffers = list_buffers!((list_buffers), -F "1");
    #[cfg(feature = "tmux_3_2")]
    let list_buffers = list_buffers!((list_buffers), -f "2");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let list_buffers = list_buffers!((list_buffers), -t "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-buffers";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lsb";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "3"]);
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let list_buffers = list_buffers.build().to_vec();

    assert_eq!(list_buffers, s);
}
