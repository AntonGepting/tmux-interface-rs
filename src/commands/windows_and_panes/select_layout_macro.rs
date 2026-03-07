// auto-generated file
//

/// Choose a specific layout for a window
///
/// # Manual
///
/// tmux >=2.7:
/// ```text
/// select-layout [-Enop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux >=2.1:
/// ```text
/// select-layout [-nop] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
///
/// tmux >=1.5:
/// ```text
/// select-layout [-np] [-t target-pane] [layout-name]
/// (alias: selectl)
/// ```
#[macro_export]
macro_rules! select_layout {
    // `[-E]`
    (@cmd ($cmd:expr) -E, $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({
            $cmd.spread()
        }) $($tail)*)
    }};

    // `[-n]`
    (@cmd ($cmd:expr) -n, $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({
            $cmd.next_layout()
        }) $($tail)*)
    }};

    // `[-o]`
    (@cmd ($cmd:expr) -o, $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({
            $cmd.last_layout()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({
            $cmd.previous_layout()
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_2_7")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    // `[layout-name]`
    (@cmd ($cmd:expr) $layout_name:expr, $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({
            $cmd.layout_name($layout_name)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SelectLayout::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::select_layout!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::select_layout!(@cmd ({ $crate::SelectLayout::new() }) $($tail)*,)
    }};
}

#[test]
fn select_layout_macro() {
    use std::borrow::Cow;

    // Choose a specific layout for a window
    //
    // # Manual
    //
    // tmux >=2.7:
    // ```text
    // select-layout [-Enop] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux >=2.1:
    // ```text
    // select-layout [-nop] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // select-layout [-np] [-t target-pane] [layout-name]
    // (alias: selectl)
    // ```

    let select_layout = select_layout!();
    #[cfg(feature = "tmux_2_7")]
    let select_layout = select_layout!((select_layout), -E);
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout!((select_layout), -n);
    #[cfg(feature = "tmux_2_1")]
    let select_layout = select_layout!((select_layout), -o);
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout!((select_layout), -p);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
    let select_layout = select_layout!((select_layout), -t "1");
    #[cfg(feature = "tmux_2_7")]
    let select_layout = select_layout!((select_layout), -t "2");
    #[cfg(feature = "tmux_1_5")]
    let select_layout = select_layout!((select_layout), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "select-layout";
    #[cfg(feature = "cmd_alias")]
    let cmd = "selectl";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_7")]
    s.push("-E");
    #[cfg(feature = "tmux_1_5")]
    s.push("-n");
    #[cfg(feature = "tmux_2_1")]
    s.push("-o");
    #[cfg(feature = "tmux_1_5")]
    s.push("-p");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_7")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_2_7")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let select_layout = select_layout.build().to_vec();

    assert_eq!(select_layout, s);
}
