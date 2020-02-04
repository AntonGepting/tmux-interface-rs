#[test]
fn target_pane_ex() {
    use crate::request::target_pane::{TargetPane, TargetPaneEx, TargetPaneToken};

    let target_pane = TargetPaneEx {
        target_window: None,
        target_pane: Some(TargetPane::Token(TargetPaneToken::Next(None))),
    };
    assert_eq!(target_pane.to_string(), ".+");
}

#[test]
fn target_pane() {
    use crate::request::target_pane::{TargetPane, TargetPaneToken};

    let target_pane = TargetPane::Token(TargetPaneToken::Next(None));
    assert_eq!(target_pane.to_string(), ".+");
    let target_pane = TargetPane::Index(1);
    assert_eq!(target_pane.to_string(), ".1");
    let target_pane = TargetPane::Id(1);
    assert_eq!(target_pane.to_string(), ".%1");
    let target_pane = TargetPane::ExactName("exact_name");
    assert_eq!(target_pane.to_string(), ".=exact_name");
    let target_pane = TargetPane::StartName("start_name");
    assert_eq!(target_pane.to_string(), ".start_name");
    let target_pane = TargetPane::StartName("fn_match");
    assert_eq!(target_pane.to_string(), ".fn_match");
    let target_pane = TargetPane::StartName("raw_str");
    assert_eq!(target_pane.to_string(), ".raw_str");
}

#[test]
fn target_pane_token() {
    use crate::request::target_pane::{TargetPane, TargetPaneToken};

    let type_token = TargetPane::Token(TargetPaneToken::Next(Some(1))).to_string();
    assert_eq!(type_token.to_string(), ".+1");
    let type_token = TargetPane::Token(TargetPaneToken::Next(None)).to_string();
    assert_eq!(type_token.to_string(), ".+");
    let type_token = TargetPane::Token(TargetPaneToken::Previous(Some(1))).to_string();
    assert_eq!(type_token.to_string(), ".-1");
    let type_token = TargetPane::Token(TargetPaneToken::Previous(None)).to_string();
    assert_eq!(type_token.to_string(), ".-");
    let type_token = TargetPane::Token(TargetPaneToken::Top).to_string();
    assert_eq!(type_token.to_string(), ".{top}");
    let type_token = TargetPane::Token(TargetPaneToken::Bottom).to_string();
    assert_eq!(type_token.to_string(), ".{bottom}");
    let type_token = TargetPane::Token(TargetPaneToken::Left).to_string();
    assert_eq!(type_token.to_string(), ".{left}");
    let type_token = TargetPane::Token(TargetPaneToken::Right).to_string();
    assert_eq!(type_token.to_string(), ".{right}");
    let type_token = TargetPane::Token(TargetPaneToken::BottomLeft).to_string();
    assert_eq!(type_token.to_string(), ".{bottom-left}");
    let type_token = TargetPane::Token(TargetPaneToken::BottomRight).to_string();
    assert_eq!(type_token.to_string(), ".{bottom-right}");
    let type_token = TargetPane::Token(TargetPaneToken::UpOf).to_string();
    assert_eq!(type_token.to_string(), ".{up-of}");
    let type_token = TargetPane::Token(TargetPaneToken::DownOf).to_string();
    assert_eq!(type_token.to_string(), ".{down-of}");
    let type_token = TargetPane::Token(TargetPaneToken::LeftOf).to_string();
    assert_eq!(type_token.to_string(), ".{left-of}");
    let type_token = TargetPane::Token(TargetPaneToken::RightOf).to_string();
    assert_eq!(type_token.to_string(), ".{right-of}");
}
