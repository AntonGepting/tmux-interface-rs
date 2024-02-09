#[test]
fn clients_ctl_test() {
    use crate::ClientsCtl;

    let clients = ClientsCtl::default().get_all().unwrap();
    dbg!(clients);
}
