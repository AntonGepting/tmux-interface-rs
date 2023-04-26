#[test]
fn get() {
    use crate::WindowsCtl;

    let windows = WindowsCtl::default().get_all().unwrap();
    dbg!(windows);
}
