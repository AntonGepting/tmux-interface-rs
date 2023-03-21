pub mod global;
pub mod local;

pub mod get_window_option;
pub mod get_window_option_value;
pub mod get_window_options;
pub mod set_window_option;
pub mod set_window_options;

pub use global::*;
pub use local::*;
// XXX: local == just window_opions

pub use get_window_option::GetWindowOption;
pub use get_window_option_value::*;
pub use get_window_options::*;
pub use set_window_option::*;
pub use set_window_options::*;

#[cfg(test)]
#[path = "."]
mod window_tests {
    //#[cfg(feature = "tmux_0_8")]
    pub mod get_window_option_tests;
    //#[cfg(feature = "tmux_0_8")]
    pub mod get_window_options_tests;

    //#[cfg(feature = "tmux_0_8")]
    pub mod set_window_option_tests;
    //#[cfg(feature = "tmux_0_8")]
    pub mod set_window_options_tests;
}

#[test]
fn window_options_111() {
    use crate::{
        GetGlobalWindowOption, GlobalWindowOptionsCtl, ShowOptions, Tmux, WindowOptions,
        WindowOptionsCtl,
    };

    let _cmd = GetGlobalWindowOption::allow_rename(Some(":"));
    let cmd = ShowOptions::new()
        .target("")
        .global()
        .option("allow-rename")
        .build();
    let output = Tmux::with_command(cmd).output().unwrap();
    let value: WindowOptions = output.to_string().parse().unwrap();
    dbg!(value);

    let value = GlobalWindowOptionsCtl::default()
        .get_allow_rename()
        .unwrap();
    dbg!(value);
}
