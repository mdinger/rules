use rules::unicode::regex::PERLS;
use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::parse::Op::*;
use rules::parse::Sign::*;
use rules::range_set;

use super::parse;
use super::new_deque;

#[test]
fn chars() {
    // Set of chars inside `[]`
    let set = vec![Char('a'), Char('b'), Char('c')].into();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![CharClass(deque, Positive)], parse(r"<[ abc ] >"));
}
#[test]
fn with_ops() {
    // Set of chars inside `[]`
    let set_a = vec![Char('a')].into();
    let set_b = vec![Char('b')].into();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set_a, Inclusive),
                               Op(Union),
                               Set(set_b, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![CharClass(deque, Positive)], parse(r"<[ a ] + [ b ] >"));
}
#[test]
fn pre_op() {
    // Set of chars inside `[]`
    let set = vec![Char('a')].into();
    // Deque of ops and sets inside `<>`.
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![CharClass(deque, Negative)], parse(r"<-[ a ] >"));
}
#[test]
fn ellipsis() {
    // Set of chars inside `[]`
    let set = vec![Range(range_set::Range('a','d'))].into();
    // Deque of ops and sets inside `<>`.
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![CharClass(deque, Positive)], parse(r"<[ a .. d ] >"));
}
#[test]
fn escape_chars() {
    // Set of chars inside `[]`. `\s` is a set itself inside the set `[]`.
    // Deque of ops and sets inside `<>`.
    let deque1 = new_deque(vec![Set(PERLS.into(), Inclusive)]);
    assert_eq!(vec![CharClass(deque1, Positive)], parse(r"<[ \s ] >"));

    // Deque of ops and sets inside `<>`. Unifying sets with opposite
    // membership isn't obvious. If -3 is 3 Exclusive and 3 is Inclusive
    // then `-3 + 3` is a union which is identical to `-(3 - 3)` = `-()`.
    // Similarly, `-1 + 7` = `-(1 - 7)` = `-1`. Thus `\s + \S` = `-\s - \s`
    // = -(\s - \s)` = `-()`. Hence, this is an exclusive set.

    let deque2 = new_deque(vec![Set(range_set::Set::new(), Exclusive)]);
    // A single class denoted by `<[]>`. No `-` so this is a positive
    // character class.
    assert_eq!(vec![CharClass(deque2, Positive)], parse(r"<[ \s \S ] >"));
}
#[test]
fn unicode() {
    // Set of chars inside `[]`
    let set = vec![Char('a'), Char('こ')].into();
    // Deque of ops and sets inside `<>`.
    let deque = new_deque(vec![Set(set, Inclusive)]);
    assert_eq!(vec![CharClass(deque, Positive)], parse(r"<[ a こ ] >"));
}
#[test]
fn multiple_char_classes() {
    // Set of chars inside `[]`
    let set_a = vec![Char('a')].into();
    let set_c = vec![Char('c')].into();

    // Deque of ops and sets inside `<>`
    let deque_a = new_deque(vec![Set(set_a, Inclusive)]);
    let deque_c = new_deque(vec![Set(set_c, Inclusive)]);

    // Multiple character classes
    assert_eq!(vec![CharClass(deque_a, Positive),
                    Char('b'),
                    CharClass(deque_c, Positive)], parse(r"<[ a ]> b <[ c ]>"));
}
