use rules::re::Regex;

#[test]
fn whitespace_ignored() {
    let re = Regex::new(r"pine
                          apple");
    assert!( re.is_match("I bought a pineapple."));

    let re = Regex::new(r"gold fish");
    assert!( re.is_match("Do you have a goldfish?"));
}
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
    let re = Regex::new(r" 'two apples' ");
    assert!( re.is_match("Are there two apples? Really?"));
    assert!(!re.is_match("Are there two grapes? Really?"));
}
#[test]
fn literals_quotes() {
    let re = Regex::new(r#" "two apples" "#);
    assert!( re.is_match("Are there two apples? Really?"));
    assert!(!re.is_match("Are there two grapes? Really?"));
}
#[test]
fn literals_regular() {
    let re = Regex::new(r#" '2' and "3" and "4" and '5' "#);
    assert!( re.is_match("Well, 2and3and4and5 there."));
    assert!(!re.is_match("Well, 5and4and3and2 there."));
}
#[test]
fn char_class_range() {
    let re = Regex::new(r"<[ a .. z ]>");
    assert!( re.is_match("a"));
    assert!( re.is_match("p"));
    assert!( re.is_match("z"));
    assert!(!re.is_match("0"));
    assert!(!re.is_match("9"));
    assert!(!re.is_match("こ"));
}
#[test]
fn char_class_multi() {
    let re = Regex::new(r"<[ 0 .. 9 ]> <[ a .. z ]>");
    assert!( re.is_match("0z"));
    assert!( re.is_match("3b"));
    assert!( re.is_match("9h"));

    // Doesn't match with a gap.
    assert!(!re.is_match("0_z"));
    assert!(!re.is_match("3_b"));
    assert!(!re.is_match("9_h"));

    // Must be in the correct order.
    assert!(!re.is_match("a3"));

    // Must be the correct case.
    assert!(!re.is_match("3A"));

    // Combines with other operators properly.
    let re = Regex::new(r"<[ 0 .. 9 ]> ' dogs'");
    assert!( re.is_match("0 dogs"));
    assert!( re.is_match("4 dogz and 3 dogs"));
    assert!( re.is_match("9 dogs more"));
    assert!( re.is_match("4 cats and 7 dogs and 2 pigs"));

    // Doesn't hit partial matches.
    assert!(!re.is_match("五 dogs")); // Japanese number 5
    assert!(!re.is_match("b dogs"));
    assert!(!re.is_match("A dogs"));
    assert!(!re.is_match("0 dogz"));
    assert!(!re.is_match("4 cats and 3 dogz"));
    assert!(!re.is_match("9 dogz and 3 cats"));
    assert!(!re.is_match("4 cats and 7 dogz and 2 pigs"));
}
#[test]
fn comments_are_ignored() {
    let before = Regex::new(r"# This is a comment
                              pineapple");
    let between = Regex::new(r"pine  # This is a comment
                               apple");
    let after = Regex::new(r"pineapple # This is a comment");
    let after_both = Regex::new(r"pine # This is a comment
                                  apple # This is second comment");

    assert!(before    .is_match("pineapple"));
    assert!(between   .is_match("pineapple"));
    assert!(after     .is_match("pineapple"));
    assert!(after_both.is_match("pineapple"));
}

