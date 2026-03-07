// auto-generated file
//

/// Put a pane into customize mode
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// customize-mode [-NZ] [-F format] [-f filter] [-t target-pane] [template]
/// ```
#[macro_export]
macro_rules! customize_mode {
    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({
            $cmd.without_option_info()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-f filter]`
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};

    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target_pane:expr, $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({
            $cmd.target_pane($target_pane)
        }) $($tail)*)
    }};

    // `[template]`
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({
            $cmd.template($template)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::CustomizeMode::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::customize_mode!(@cmd ({ $crate::CustomizeMode::new() }) $($tail)*,)
    }};
}

#[test]
fn customize_mode_macro() {
    use std::borrow::Cow;

    // Put a pane into customize mode
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // customize-mode [-NZ] [-F format] [-f filter] [-t target-pane] [template]
    // ```

    let customize_mode = customize_mode!();
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode!((customize_mode), -N);
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode!((customize_mode), -Z);
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode!((customize_mode), -F "");
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode!((customize_mode), -f "");
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode!((customize_mode), -t "");
    #[cfg(feature = "tmux_3_2")]
    let customize_mode = customize_mode!((customize_mode), "");

    let cmd = "customize-mode";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-N");
    #[cfg(feature = "tmux_3_2")]
    s.push("-Z");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-F", ""]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-f", ""]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-t", ""]);
    #[cfg(feature = "tmux_3_2")]
    s.push("");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let customize_mode = customize_mode.build().to_vec();

    assert_eq!(customize_mode, s);
}
