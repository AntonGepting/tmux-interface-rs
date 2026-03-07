// auto-generated file
//

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// choose-client [-NryZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.2:
/// ```text
/// choose-client [-NrZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=3.1:
/// ```text
/// choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.7:
/// ```text
/// choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=2.6:
/// ```text
/// choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux >=1.7:
/// ```text
/// choose-client [-F format] [-t target-window] [template]
/// ```
///
/// tmux >=1.0:
/// ```text
/// choose-client  [-t target-window] [template]
/// ```
#[macro_export]
macro_rules! choose_client {
    // `[-N]`
    (@cmd ($cmd:expr) -N, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.without_preview()
        }) $($tail)*)
    }};

    // `[-r]`
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.reverse_sort_order()
        }) $($tail)*)
    }};

    // `[-y]`
    (@cmd ($cmd:expr) -y, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.disable_confirmation()
        }) $($tail)*)
    }};

    // `[-Z]`
    (@cmd ($cmd:expr) -Z, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.zoom()
        }) $($tail)*)
    }};

    // `[-F format]`
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};

    // `[-f filter]`
    (@cmd ($cmd:expr) -f $filter:expr, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.filter($filter)
        }) $($tail)*)
    }};

    // `[-K key-format]`
    (@cmd ($cmd:expr) -K $key_format:expr, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.key_format($key_format)
        }) $($tail)*)
    }};

    // `[-O sort-order]`
    (@cmd ($cmd:expr) -O $sort_order:expr, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            $cmd.sort_order($sort_order)
        }) $($tail)*)
    }};

    // `[-t target-window]`
    // `[-t target-pane]`
    (@cmd ($cmd:expr) -t $target:expr, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
            #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
            {
                $cmd.target_window($target)
            }
            #[cfg(feature = "tmux_2_6")]
            {
                $cmd.target_pane($target)
            }
        }) $($tail)*)
    }};

    // `[template]`
    (@cmd ($cmd:expr) $template:expr, $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({
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
        $crate::ChooseClient::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::choose_client!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::choose_client!(@cmd ({ $crate::ChooseClient::new() }) $($tail)*,)
    }};
}

#[test]
fn choose_client_macro() {
    use std::borrow::Cow;

    // Put a pane into client mode, allowing a client to be selected interactively from a list
    //
    // # Manual
    //
    // tmux >=3.6:
    // ```text
    // choose-client [-NryZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=3.2:
    // ```text
    // choose-client [-NrZ] [-F format] [-f filter] [-K key-format] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=3.1:
    // ```text
    // choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=2.7:
    // ```text
    // choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=2.6:
    // ```text
    // choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    // ```
    //
    // tmux >=1.7:
    // ```text
    // choose-client [-F format] [-t target-window] [template]
    // ```
    //
    // tmux >=1.0:
    // ```text
    // choose-client  [-t target-window] [template]
    // ```

    let choose_client = choose_client!();
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client!((choose_client), -N);
    #[cfg(feature = "tmux_3_1")]
    let choose_client = choose_client!((choose_client), -r);
    #[cfg(feature = "tmux_3_6")]
    let choose_client = choose_client!((choose_client), -y);
    #[cfg(feature = "tmux_2_7")]
    let choose_client = choose_client!((choose_client), -Z);
    #[cfg(feature = "tmux_1_7")]
    let choose_client = choose_client!((choose_client), -F "1");
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client!((choose_client), -f "2");
    #[cfg(feature = "tmux_3_2")]
    let choose_client = choose_client!((choose_client), -K "3");
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client!((choose_client), -O "4");
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    let choose_client = choose_client!((choose_client), -t "5");
    #[cfg(feature = "tmux_2_6")]
    let choose_client = choose_client!((choose_client), -t "6");
    #[cfg(feature = "tmux_1_5")]
    let choose_client = choose_client!((choose_client), "7");

    let cmd = "choose-client";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_6")]
    s.push("-N");
    #[cfg(feature = "tmux_3_1")]
    s.push("-r");
    #[cfg(feature = "tmux_3_6")]
    s.push("-y");
    #[cfg(feature = "tmux_2_7")]
    s.push("-Z");
    #[cfg(feature = "tmux_1_7")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-f", "2"]);
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-K", "3"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-O", "4"]);
    #[cfg(all(feature = "tmux_1_5", not(feature = "tmux_2_6")))]
    s.extend_from_slice(&["-t", "5"]);
    #[cfg(feature = "tmux_2_6")]
    s.extend_from_slice(&["-t", "6"]);
    #[cfg(feature = "tmux_1_5")]
    s.push("7");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();
    let choose_client = choose_client.build().to_vec();

    assert_eq!(choose_client, s);
}
