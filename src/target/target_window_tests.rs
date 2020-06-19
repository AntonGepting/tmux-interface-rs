#[test]
fn target_window_ex() {
    use crate::{TargetWindow, TargetWindowExt, TargetWindowToken};

    let _target_window = TargetWindowExt {
        ..Default::default()
    };

    let target_window = TargetWindowExt {
        session: None,
        window: Some(TargetWindow::Token(TargetWindowToken::Start)),
    };
    assert_eq!(target_window.to_string(), ":^");
}

#[test]
fn target_window() {
    use crate::{TargetWindow, TargetWindowToken};

    let target_window = TargetWindow::Token(TargetWindowToken::Start).to_string();
    assert_eq!(target_window, ":^");
    let target_window = TargetWindow::Index(1).to_string();
    assert_eq!(target_window, ":1");
    let target_window = TargetWindow::Id(1).to_string();
    assert_eq!(target_window, ":@1");
    let target_window = TargetWindow::ExactName("exact_name").to_string();
    assert_eq!(target_window, ":=exact_name");
    let target_window = TargetWindow::StartName("start_name").to_string();
    assert_eq!(target_window, ":start_name");
    let target_window = TargetWindow::FnMatch("fn_match").to_string();
    assert_eq!(target_window, ":fn_match");
    let target_window = TargetWindow::Raw("raw").to_string();
    assert_eq!(target_window, "raw");
}

#[test]
fn target_window_type_token() {
    use crate::{TargetWindow, TargetWindowToken};

    let type_token = TargetWindow::Token(TargetWindowToken::Start).to_string();
    assert_eq!(type_token, ":^");
    let type_token = TargetWindow::Token(TargetWindowToken::End).to_string();
    assert_eq!(type_token, ":$");
    let type_token = TargetWindow::Token(TargetWindowToken::Last).to_string();
    assert_eq!(type_token, ":!");
    let type_token = TargetWindow::Token(TargetWindowToken::Next(Some(1))).to_string();
    assert_eq!(type_token, ":+1");
    let type_token = TargetWindow::Token(TargetWindowToken::Previous(Some(1))).to_string();
    assert_eq!(type_token, ":-1");
}
