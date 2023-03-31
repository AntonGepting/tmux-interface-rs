#[test]
fn global_window_options_ctl_get_all() {
    use crate::{GlobalWindowOptionsCtl, WindowOptionsCtl};

    let output = GlobalWindowOptionsCtl::default().get_all().unwrap();
    dbg!(&output);
}

// #[test]
// fn global_window_options_ctl_set_all() {
// use crate::{GlobalWindowOptionsCtl, WindowOptions, WindowOptionsCtl};
//
// let window_options = WindowOptions::default();
// let output = GlobalWindowOptionsCtl::default()
// .set_all(window_options)
// .unwrap();
// dbg!(&output);
// }
//
