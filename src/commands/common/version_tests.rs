#[test]
fn from_str() {
    use crate::Version;

    let examples = &[
        "tmux next-3.2",
        "tmux 3.1b",
        "tmux 3.1a",
        "tmux 3.1",
        "tmux 3.0a",
        "tmux 3.0",
        "tmux 2.9a",
        "tmux 2.9",
        "tmux 2.8",
        "tmux 2.7-rc",
        "tmux 2.6",
        "tmux 2.5",
        "tmux 2.4",
        "tmux 2.3",
        "tmux 2.2",
        "tmux 2.1",
        "tmux 2.0",
        "tmux 1.9a",
        "tmux 1.9",
        "tmux 1.8",
        "tmux 1.7",
        "tmux 1.6",
        "tmux 1.5",
        "tmux 1.4",
        "tmux 1.3",
        "tmux 1.2",
        "tmux 1.1",
        "tmux 1.0",
        "tmux 0.9",
        "tmux 0.8",
    ];

    for example in examples {
        let version: Version = example.parse().unwrap();
        assert_eq!(version.prog_name, "tmux");
        //assert!(version.major >= 0);
        //assert!(version.minor >= 0);
    }
}

//#[test]
//fn from_str_error() {
//use crate::{Error, Version};

////let err = "tmux a2.8a".parse::<Version>().unwrap_err();
////assert!(if let Error::ParseInt(_) = err {
////true
////} else {
////false
////})
//}
