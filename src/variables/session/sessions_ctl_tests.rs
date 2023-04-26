#[test]
fn get() {
    use crate::SessionsCtl;

    let sessions = SessionsCtl::default().get_all().unwrap();
    dbg!(sessions);
}
