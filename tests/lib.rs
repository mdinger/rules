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
    // chars match correctly inside `'` sets.
    match_succeed("f're''d'", "this_fred_and_that");
  
    match_fail("fred", "coconut");
    match_fail("fred", "friend");
    match_fail("fred", "this_fiend_and");
}

#[test]
fn dot_matches() {
    match_succeed(r"d.t", r"dot");
    match_succeed(r"d..", r"d  \n");
    match_succeed(r"d..", r"d\t\t");
    // d + 2 newlines
    match_succeed(r"d..", r"d

");
}

#[test]
fn digits_match() {
    match_succeed(r"\d . apples", r"7 apples");
    match_succeed(r"\d\d / \d\d / \d\d\d\d", r"07/14/1963");
    // 4 non-digits
    match_succeed(r"\D\D\D\D", r"fred");
}

#[test]
fn horizontal_whitespace_matches() {
	// 2 spaces
    match_succeed(r"\h\h", r"  ");
    // 3 colons separated by tabs
    match_succeed(r"\: \h \: \h \:", r":	:	:");
    // 3 colons separated by actual tabs
    match_succeed(r"':	:	:'", r":	:	:");
    // paper cut surrounded by 3 non-whitespace characters
    match_succeed(r"\H paper \H cut \H", r":paper:cut:");
}

#[test]
fn vertical_whitespace_matches() {
    match_succeed(r"use \n newline \n character", r"use
newline
character");
}

#[test]
fn whitespace_matches() {
    match_succeed(r"match \t tabs \n newlines", r"match	tabs
newlines");
	// Match 3 non-tab, non-newline
    match_succeed(r"\T\N\N", r"are");
	// Same as previous but match actual characters (not escaped sequences)
    match_succeed(r"'match	tabs
newlines'", r"match	tabs
newlines");
	// `\s` matches all whitespace
    match_succeed(r"match \s tabs \s newlines", r"match	tabs
newlines");
	// Match 3 non whitespace
    match_succeed(r"\S\S\S", r"are");
}

#[test]
fn words_match() {
	// `\w` matches digits, characters, underscores.
    match_succeed(r"\w\w\w \w \w", r"L8t_e");
    // Match 3 non-word characters
    match_succeed(r"\W \W \W", r"&*$");
}
