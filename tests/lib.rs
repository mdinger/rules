extern crate rules;

use rules::Rule;

fn match_succeed(regex: &str, s: &str) {
    let rule = match Rule::new(regex) {
        Ok(rule) => rule,
        Err(err) => panic!("{}", err),
    };
    
    assert!(rule.is_match(s));
}

fn match_fail(regex: &str, s: &str) {
    let rule = match Rule::new(regex) {
        Ok(rule) => rule,
        Err(err) => panic!("{}", err),
    };
    
    assert!(!rule.is_match(s));
}

#[test]
fn matches_substr() {
    match_succeed("fred", "fred");
    match_succeed("fred", "this_fred");
    match_succeed("fred", "fred_that");
    match_succeed("fred", "this_fred_and_that");
    match_succeed("fred", "this_fiend_and_fred_and_that");
  
    match_fail("fred", "coconut");
    match_fail("fred", "friend");
    match_fail("fred", "this_fiend_and");
}


