#[test]
fn local_window_options_ctl_get_all() {
    use crate::{LocalWindowOptionsCtl, WindowOptionsCtl};

    let output = LocalWindowOptionsCtl::default().get_all().unwrap();
    dbg!(&output);
}

// #[test]
// fn local_window_options_ctl_set_all() {
// use crate::{LocalWindowOptionsCtl, WindowOptions, WindowOptionsCtl};
//
// let window_options = WindowOptions::default();
// let output = LocalWindowOptionsCtl::default()
// .set_all(window_options)
// .unwrap();
// dbg!(&output);
// }
//
