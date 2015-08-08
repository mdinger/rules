use rules::re::Regex;

#[test]
fn chars() {
    let re = Regex::new(r"Apple");
    assert!( re.is_match("I have an Apple. Do you?"));
    assert!(!re.is_match("I have an Pear. Do you?"));
}
#[test]
fn whitespace_escapes() {
    let re = Regex::new(r"ping\ pong");
    assert!( re.is_match("Do you like ping pong? I do."));
    assert!(!re.is_match("Do you like golf? I do."));
}
#[test]
fn literals_ticks() {
    let re = Regex::new(r"'two apples'");
    assert!( re.is_match("Are there two apples? Really?"));
    assert!(!re.is_match("Are there two grapes? Really?"));
}
#[test]
fn literals_quotes() {
    let re = Regex::new(r#""two apples""#);
    assert!( re.is_match("Are there two apples? Really?"));
    assert!(!re.is_match("Are there two grapes? Really?"));
}
#[test]
fn literals_regular() {
    let re = Regex::new(r#"'2' and "3" and "4" and '5'"#);
    assert!( re.is_match("Well, 2and3and4and5 there."));
    assert!(!re.is_match("Well, 5and4and3and2 there."));
}
#[test]
fn char_class_range() {
    let re = Regex::new(r"<[a .. z]>");
    assert!( re.is_match("a"));
    assert!( re.is_match("p"));
    assert!( re.is_match("z"));
    assert!(!re.is_match("0"));
    assert!(!re.is_match("9"));
    assert!(!re.is_match("こ"));
}
