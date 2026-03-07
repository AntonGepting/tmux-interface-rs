// auto-generated file
//

/// Put a window into list choice mode
///
/// # Manual
///
/// tmux >=1.7 && <=1.9a:
/// ```text
/// choose-list [-l items] [-t target-window] [template]
/// ```
#[macro_export]
macro_rules! choose_list {
    // `[-l items]`
    (@cmd ($cmd:expr) -l $items:expr, $($tail:tt)*) => {{
        $crate::choose_list!(@cmd ({
            $cmd.items($items)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    (@cmd ($cmd:expr) -t $target_window:expr, $($tail:tt)*) => {{
        $crate::choose_list!(@cmd ({
            $cmd.target_window($target_window)
        }) $($tail)*)
    }};

    // `[template]`
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::choose_list!(@cmd ({
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
        $crate::ChooseList::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_list!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_list!(@cmd ({ $crate::ChooseList::new() }) $($tail)*,)
    }};
}

#[test]
fn choose_list_macro() {
    use std::borrow::Cow;

    // Put a window into list choice mode
    //
    // # Manual
    //
    // tmux >=1.7 && <=1.9a:
    // ```text
    // choose-list [-l items] [-t target-window] [template]
    // ```

    let choose_list = choose_list!();
    #[cfg(feature = "tmux_1_7")]
    let choose_list = choose_list!((choose_list), -l "1");
    #[cfg(feature = "tmux_1_7")]
    let choose_list = choose_list!((choose_list), -t "2");
    #[cfg(feature = "tmux_1_7")]
    let choose_list = choose_list!((choose_list), "3");

    let cmd = "choose-list";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-l", "1"]);
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_7")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let choose_list = choose_list.build().to_vec();

    assert_eq!(choose_list, s);
}
