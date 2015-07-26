use rules::re::Regex;

#[test]
fn chars() {
    let mut re = Regex::new(r"Apple");
    assert!(re.is_match("I have an Apple. Do you?"));
}
#[test]
fn whitespace_escapes() {
    let mut re = Regex::new(r"ping\ pong");
    assert!(re.is_match("Do you like ping pong? I do."));
}
/*#[test]
fn literals_ticks() {
    let mut re = Regex::new(r"'two apples'");
    assert!(re.is_match("Are there two apples? Really?"));
}
#[test]
fn literals_quotes() {
    let mut re = Regex::new(r#""two apples""#);
    assert!(re.is_match("Are there two apples? Really?"));
}*/
