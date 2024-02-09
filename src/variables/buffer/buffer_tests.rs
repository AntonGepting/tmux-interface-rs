#[test]
fn buffer_default() {
    use crate::Buffer;

    let buffer = Buffer {
        ..Default::default()
    };

    let buffer_orig = Buffer {
        #[cfg(feature = "tmux_2_6")]
        created: None,
        #[cfg(feature = "tmux_2_3")]
        name: None,
        #[cfg(feature = "tmux_1_7")]
        sample: None,
        #[cfg(feature = "tmux_1_7")]
        size: None,
    };

    assert_eq!(buffer_orig, buffer);
}

#[test]
fn buffer_parse() {
    use crate::Buffer;
    use std::str::FromStr;

    let buffer_vec = vec![
        // buffer_created - Time buffer created
        #[cfg(feature = "tmux_2_6")]
        "1707496726",
        // buffer_name - Name of buffer
        #[cfg(feature = "tmux_2_3")]
        "foo",
        // buffer_sample - First 50 characters from the specified buffer
        #[cfg(feature = "tmux_1_7")]
        "bar",
        // buffer_size - Size of the specified buffer in bytes
        #[cfg(feature = "tmux_1_7")]
        "3",
    ];
    let buffer_str = buffer_vec.join(":");
    let buffer = Buffer::from_str(&buffer_str).unwrap();

    let buffer_orig = Buffer {
        #[cfg(feature = "tmux_2_6")]
        created: Some(1707496726),
        #[cfg(feature = "tmux_2_3")]
        name: Some("foo".to_string()),
        #[cfg(feature = "tmux_1_7")]
        sample: Some("bar".to_string()),
        #[cfg(feature = "tmux_1_7")]
        size: Some(3),
    };

    assert_eq!(buffer_orig, buffer);
}
