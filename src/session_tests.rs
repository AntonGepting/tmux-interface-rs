#[test]
fn stack_parse() {
    use regex::Regex;
    let regex = Regex::new(r"^:([\d,]*):$").unwrap();
    assert!(regex.is_match(":3,2,1:"));
}


#[test]
fn parse() {
    use crate::Session;

    let session = Session::parse(":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3").unwrap();
    assert_eq!(session.name, Some("0".to_string()));
    assert_eq!(session.id, Some(0));
}




#[test]
fn vec_variables() {

    const SDF: [(&str, &str); 15] = [
        ("session_alerts", r"(\w+)?"),
        ("session_attached", r"(\d+)?"),
        ("session_activity", r"(\d+)?"),
        ("session_created", r"(\d+)?"),
        ("session_format", r"(\w+)?"),
        ("session_last_attached", r"(\d+)?"),
        ("session_group", r"(\w+)?"),
        ("session_group_size", r"(\w+)?"),
        ("session_group_list", r"(\w+)?"),
        ("session_grouped", r"(\w+)?"),
        ("session_id", r"\$(\d+)?"),
        ("session_many_attached", r"(\w+)?"),
        ("session_name", r"(\w+)?"),
        ("session_stack", r"([\w,]*)?"),
        ("session_windows", r"(\d+)?"),
    ];

    let a = r"^(\w+)?:(\d+)?:(\d+)?:(\d+)?:(\w+)?:(\d+)?:(\w+)?:(\w+)?:(\w+)?:(\w+)?:\$(\d+)?:(\w+)?:(\w+)?:([\w,]*)?:(\d+)?$";
    //let variables: Vec<&str> = vec.iter().map(|t| t.0).collect().join(":");
    //let variables = vec.iter().map(|t| format!("#{{{}}}", t.0)).collect::<Vec<String>>().join(":");
    let regex = format!("^{}$", SDF.iter().map(|t| t.1).collect::<Vec<&str>>().join(":"));
    assert_eq!(a, regex);
}
