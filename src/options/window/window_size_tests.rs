#[test]
#[cfg(feature = "tmux_2_9")]
fn window_size() {
    use crate::WindowSize;
    assert_eq!(WindowSize::Largest.to_string(), "largest");
    assert_eq!(WindowSize::Smallest.to_string(), "smallest");
    assert_eq!(WindowSize::Manual.to_string(), "manual");
    #[cfg(feature = "tmux_3_1")]
    assert_eq!(WindowSize::Latest.to_string(), "latest");
}
