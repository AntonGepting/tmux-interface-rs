#[test]
fn parse() {
    use crate::SessionStack;

    let s = "3,2,1";
    let session_stack = s.parse::<SessionStack>().unwrap();
    let session_stack_origin = SessionStack(vec![3, 2, 1]);
    assert_eq!(session_stack, session_stack_origin);
}
