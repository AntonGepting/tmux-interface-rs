#[test]
fn buffers_parse() {
    use crate::Buffers;
    use std::str::FromStr;

    let buffer0_vec = vec![
        #[cfg(feature = "tmux_2_6")]
        "1707496726",
        #[cfg(feature = "tmux_2_3")]
        "foo1",
        #[cfg(feature = "tmux_1_7")]
        "bar",
        #[cfg(feature = "tmux_1_7")]
        "3",
    ];
    let buffer1_vec = vec![
        #[cfg(feature = "tmux_2_6")]
        "1707496726",
        #[cfg(feature = "tmux_2_3")]
        "foo2",
        #[cfg(feature = "tmux_1_7")]
        "bar",
        #[cfg(feature = "tmux_1_7")]
        "3",
    ];

    let buffer0_str = buffer0_vec.join(":");
    let buffer1_str = buffer1_vec.join(":");
    let buffers_str = format!("{}\n{}", buffer0_str, buffer1_str);
    let buffers = Buffers::from_str(&buffers_str).unwrap();

    #[cfg(feature = "tmux_2_3")]
    assert_eq!(buffers[0].name, Some("foo1".to_string()));
    #[cfg(feature = "tmux_2_3")]
    assert_eq!(buffers[1].name, Some("foo2".to_string()));
}
