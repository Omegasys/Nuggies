#[test]
fn dummy_cli_parse_search() {
    let args = vec!["nuggies", "search", "firefox"];

    assert_eq!(args[1], "search");
    assert_eq!(args[2], "firefox");
}

#[test]
fn dummy_cli_parse_install() {
    let args = vec!["nuggies", "install", "vim"];

    assert_eq!(args[1], "install");
    assert_eq!(args[2], "vim");
}
