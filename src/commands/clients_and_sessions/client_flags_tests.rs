#[test]
fn to_string() {
    use crate::ClientFlags;

    let flags = ClientFlags {
        active_pane: Some(true),
        ignore_size: Some(true),
        no_output: Some(true),
        pause_after: Some(1),
        read_only: Some(true),
        wait_exit: Some(true),
    };

    assert_eq!(
        flags.to_string(),
        "active-pane,ignore-size,no-output,pause-after=1,read-only,wait-exit"
    );
}
