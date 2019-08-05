#[test]
fn bool() {
    //let a = "1".parse::<bool>().unwrap();
    let a = "1".parse::<usize>().unwrap();
    let b = if a == 1 {
        true
    } else {
        false
    };
    assert_eq!(b, true);
}


#[test]
fn parse() {
    use crate::Pane;

    let pane_str = "1'1'1'1'1'63'bash'/home/user'0''1'64'%0'0'0'0'0'0'0''1945'0'176'''0'8,16,24,32,40,48,56,64,72,80,88,96,104,112,120,128,136,144,152,160,168,176'asus'0'/dev/pts/2'177";
    let pane: Pane = pane_str.parse().unwrap();
    assert_eq!(pane.current_path, Some("/home/user".to_string()));
    assert_eq!(pane.tty, Some("/dev/pts/2".to_string()));
}

