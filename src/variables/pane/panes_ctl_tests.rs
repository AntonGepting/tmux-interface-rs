#[test]
fn get() {
    use crate::PanesCtl;

    let panes = PanesCtl::default().get_all().unwrap();
    dbg!(panes);
}
