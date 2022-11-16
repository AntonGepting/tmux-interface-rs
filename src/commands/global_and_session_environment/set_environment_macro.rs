/// Structure for setting or unsetting an environment variable
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// set-environment [-Fhgru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
///
/// tmux ^1.0:
/// ```text
/// set-environment [-gru] [-t target-session] name [value]
/// (alias: setenv)
/// ```
// FIXME: two parameters: name value
#[macro_export]
macro_rules! set_environment {
    // `[-F]` - value is expanded as a format
    (@cmd ($cmd:expr) -F, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.all()
        }) $($tail)*)
    }};
    // `[-h]` - marks the variable as hidden
    (@cmd ($cmd:expr) -h, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.hidden()
        }) $($tail)*)
    }};
    // `[-g]` - make change in the global environment
    (@cmd ($cmd:expr) -g, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.global()
        }) $($tail)*)
    }};
    // `[-r]` - remove the variable from the environment before starting a new process
    (@cmd ($cmd:expr) -r, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.remove()
        }) $($tail)*)
    }};
    // `[-u]` - unset a variable
    (@cmd ($cmd:expr) -u, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.unset()
        }) $($tail)*)
    }};
    // `[-t target-session]` - target-session
    (@cmd ($cmd:expr) -t $target_session:expr, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.target_session($target_session)
        }) $($tail)*)
    }};
    // `name`
    (@cmd ($cmd:expr) $name:expr, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.name($name)
        }) $($tail)*)
    }};
    // `[value]` - specify the value
    (@cmd ($cmd:expr) $value:expr, $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({
            $cmd.value($value)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::SetEnvironment::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::set_environment!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::set_environment!(@cmd ({ $crate::SetEnvironment::new() }) $($tail)*,)
    }};

}

#[test]
fn set_environment_macro() {
    use crate::TargetSession;
    use std::borrow::Cow;

    // Structure for setting or unsetting an environment variable
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // set-environment [-Fhgru] [-t target-session] name [value]
    // (alias: setenv)
    // ```
    //
    // tmux ^1.0:
    // ```text
    // set-environment [-gru] [-t target-session] name [value]
    // (alias: setenv)
    // ```
    let target_session = TargetSession::Raw("1").to_string();

    let set_environment = set_environment!();
    #[cfg(feature = "tmux_3_2")]
    let set_environment = set_environment!((set_environment), -F);
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment!((set_environment), -h);
    #[cfg(feature = "tmux_3_2")]
    let set_environment = set_environment!((set_environment), -g);
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment!((set_environment), -r);
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment!((set_environment), -u);
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment!((set_environment), -t & target_session);
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment!((set_environment), "2");
    #[cfg(feature = "tmux_1_0")]
    let set_environment = set_environment!((set_environment), "3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "set-environment";
    #[cfg(feature = "cmd_alias")]
    let cmd = "setenv";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_3_2")]
    s.push("-F");
    #[cfg(feature = "tmux_3_2")]
    s.push("-h");
    #[cfg(feature = "tmux_1_0")]
    s.push("-g");
    #[cfg(feature = "tmux_1_0")]
    s.push("-r");
    #[cfg(feature = "tmux_1_0")]
    s.push("-u");
    #[cfg(feature = "tmux_1_0")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_0")]
    s.push("2");
    #[cfg(feature = "tmux_1_0")]
    s.push("3");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let set_environment = set_environment.build().to_vec();

    assert_eq!(set_environment, s);
}
