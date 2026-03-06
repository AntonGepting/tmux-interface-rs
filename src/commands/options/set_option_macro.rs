// auto-generated file
//

/// Set a pane/window/session/server option
///
/// # Manual
///
/// tmux >=3.2:
/// ```text
/// set-option [-aFgopqsuUw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux >=3.0a:
/// ```text
/// set-option [-aFgopqsuw] [-t target-pane] option value
/// (alias: set)
/// ```
///
/// tmux >=2.6:
/// ```text
/// set-option [-aFgoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=1.8:
/// ```text
/// set-option [-agoqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=1.7:
/// ```text
/// set-option [-agqsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=1.5:
/// ```text
/// set-option [-agsuw] [-t target-session | target-window] option value
/// (alias: set)
/// ```
///
/// tmux >=0.8:
/// ```text
/// set-option [-gu] [-t target-session] option value
/// (alias: set)
/// ```
#[macro_export]
macro_rules! set_option {
    // `[-a]`
    (@cmd ($cmd:expr) -a, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.append()
        }) $($tail)*)
    }};

    // `[-F]`
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.format()
        }) $($tail)*)
    }};

    // `[-g]`
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};

    // `[-o]`
    (@cmd ($cmd:expr) -o, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.not_overwrite()
        }) $($tail)*)
    }};

    // `[-p]`
    (@cmd ($cmd:expr) -p, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.pane()
        }) $($tail)*)
    }};

    // `[-q]`
    (@cmd ($cmd:expr) -q, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.quiet()
        }) $($tail)*)
    }};

    // `[-s]`
    (@cmd ($cmd:expr) -s, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.server()
        }) $($tail)*)
    }};

    // `[-u]`
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.unset()
        }) $($tail)*)
    }};

    // `[-U]`
    (@cmd ($cmd:expr) -U, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.unset_on_all()
        }) $($tail)*)
    }};

    // `[-w]`
    (@cmd ($cmd:expr) -w, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.window()
        }) $($tail)*)
    }};

    // `[-t target-session]`
    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
            {
                $cmd.target_session($target)
            }
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_3_0a")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};


    // `[option]`
    (@cmd ($cmd:expr) $option:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.option($option)
        }) $($tail)*)
    }};

    // `[value]`
    (@cmd ($cmd:expr) $value:expr, $($tail:tt)*) => {{
        $crate::set_option!(@cmd ({
            $cmd.value($value)
        }) $($tail)*)
    }};

    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter: {}", $unknown);
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SetOption::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::set_option!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::set_option!(@cmd ({ $crate::SetOption::new() }) $($tail)*,)
    }};
}

#[test]
fn set_option_macro() {
    use std::borrow::Cow;

    // Set a pane/window/session/server option
    //
    // # Manual
    //
    // tmux >=3.2:
    // ```text
    // set-option [-aFgopqsuUw] [-t target-pane] option value
    // (alias: set)
    // ```
    //
    // tmux >=3.0a:
    // ```text
    // set-option [-aFgopqsuw] [-t target-pane] option value
    // (alias: set)
    // ```
    //
    // tmux >=2.6:
    // ```text
    // set-option [-aFgoqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux >=1.8:
    // ```text
    // set-option [-agoqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux >=1.7:
    // ```text
    // set-option [-agqsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux >=1.5:
    // ```text
    // set-option [-agsuw] [-t target-session | target-window] option value
    // (alias: set)
    // ```
    //
    // tmux >=0.8:
    // ```text
    // set-option [-gu] [-t target-session] option value
    // (alias: set)
    // ```

    let set_option = set_option!();
    #[cfg(feature = "tmux_1_5")]
    let set_option = set_option!((set_option), -a);
    #[cfg(feature = "tmux_2_6")]
    let set_option = set_option!((set_option), -F);
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), -g);
    #[cfg(feature = "tmux_1_8")]
    let set_option = set_option!((set_option), -o);
    #[cfg(feature = "tmux_3_0a")]
    let set_option = set_option!((set_option), -p);
    #[cfg(feature = "tmux_1_7")]
    let set_option = set_option!((set_option), -q);
    #[cfg(feature = "tmux_1_5")]
    let set_option = set_option!((set_option), -s);
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), -u);
    #[cfg(feature = "tmux_3_2")]
    let set_option = set_option!((set_option), -U);
    #[cfg(feature = "tmux_1_5")]
    let set_option = set_option!((set_option), -w);
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    let set_option = set_option!((set_option), -t "1");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
    let set_option = set_option!((set_option), -t "2");
    #[cfg(feature = "tmux_3_0a")]
    let set_option = set_option!((set_option), -t "3");
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), "4");
    #[cfg(feature = "tmux_0_8")]
    let set_option = set_option!((set_option), "5");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-option";
    #[cfg(feature = "cmd_alias")]
    let cmd = "set";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_5")]
    s.push("-a");
    #[cfg(feature = "tmux_2_6")]
    s.push("-F");
    #[cfg(feature = "tmux_0_8")]
    s.push("-g");
    #[cfg(feature = "tmux_1_8")]
    s.push("-o");
    #[cfg(feature = "tmux_3_0a")]
    s.push("-p");
    #[cfg(feature = "tmux_1_7")]
    s.push("-q");
    #[cfg(feature = "tmux_1_5")]
    s.push("-s");
    #[cfg(feature = "tmux_0_8")]
    s.push("-u");
    #[cfg(feature = "tmux_3_2")]
    s.push("-U");
    #[cfg(feature = "tmux_1_5")]
    s.push("-w");
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_3_0a")))]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_3_1")]
    s.extend_from_slice(&["-t", "3"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("4");
    #[cfg(feature = "tmux_0_8")]
    s.push("5");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let set_option = set_option.build().to_vec();

    assert_eq!(set_option, s);
}
