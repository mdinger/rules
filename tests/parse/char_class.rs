use rules::unicode::regex::PERLS;
use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::parse::Op::*;
use rules::parse::ToCharSet;

use super::parse;
use super::new_deque;

#[test]
fn chars() {
    // Set of chars inside `[]`
    let set = vec![Char('a'), Char('b'), Char('c')].to_char_set();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque)], parse(r"< [ abc ] >"));
}
#[test]
fn with_ops() {
    // Set of chars inside `[]`
    let set_a = vec![Char('a')].to_char_set();
    let set_b = vec![Char('b')].to_char_set();
    // Deque of ops and sets inside `<>`
    let deque = new_deque(vec![Set(set_a, Inclusive),
                               Op(Union),
                               Set(set_b, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque)], parse(r"< [ a ] + [ b ] >"));
}
#[test]
fn pre_op() {
    // Set of chars inside `[]`
    let set = vec![Char('a')].to_char_set();
    // Deque of ops and sets inside `<>`. Pre-ops get an `Empty`
    // prepended so binary ops can be applied to them.
    let deque = new_deque(vec![Empty,
                               Op(Difference),
                               Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque)], parse(r"< - [ a ] >"));
}
#[test]
fn post_op() {
    // Set of chars inside `[]`
    let set = vec![Char('a')].to_char_set();
    // Deque of ops and sets inside `<>`. Post-ops get an `Empty`
    // appended so binary ops can be applied to them.
    let deque = new_deque(vec![Set(set, Inclusive),
                               Op(Union),
                               Empty]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque)], parse(r"< [ a ] + >"));
}
#[test]
fn ellipsis() {
    // Set of chars inside `[]`
    let set = vec![Range('a','d')].to_char_set();
    // Deque of ops and sets inside `<>`.
    let deque = new_deque(vec![Set(set, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque)], parse(r"< [ a .. d ] >"));
}
#[test]
fn escape_chars() {
    // Set of chars inside `[]`. `\s` is a set itself inside the set `[]`.
    // Escaped character sets may be inclusive or exclusive and need to
    // keep their own parameters. So they are stored whole inside the outer
    // set to eventually be flattened into a single set after parsing.
    let set1 = vec![Set(PERLS.to_char_set(), Inclusive)].to_char_set();
    // Deque of ops and sets inside `<>`.
    let deque1 = new_deque(vec![Set(set1, Inclusive)]);
    assert_eq!(vec![Class(deque1)], parse(r"< [ \s ] >"));

    // Without the superset, these 2 would merge so they are stored whole
    // until later.
    let set2 = vec![Set(PERLS.to_char_set(), Inclusive),
                    Set(PERLS.to_char_set(), Exclusive)].to_char_set();
    // Deque of ops and sets inside `<>`.
    let deque2 = new_deque(vec![Set(set2, Inclusive)]);
    // A single class denoted by `<[]>`
    assert_eq!(vec![Class(deque2)], parse(r"< [ \s \S ] >"));
}
#[test]
fn unicode() {
    // Set of chars inside `[]`
    let set = vec![Char('a'), Char('こ')].to_char_set();
    // Deque of ops and sets inside `<>`.
    let deque = new_deque(vec![Set(set, Inclusive)]);
    assert_eq!(vec![Class(deque)], parse(r"< [ a こ ] >"));
}
#[test]
fn multiple_char_classes() {
    // Set of chars inside `[]`
    let set_a = vec![Char('a')].to_char_set();
    let set_c = vec![Char('c')].to_char_set();

    // Deque of ops and sets inside `<>`
    let deque_a = new_deque(vec![Set(set_a, Inclusive)]);
    let deque_c = new_deque(vec![Set(set_c, Inclusive)]);

    // Multiple character classes
    assert_eq!(vec![Class(deque_a),
                    Char('b'),
                    Class(deque_c)], parse(r"<[ a ]> b <[ c ]>"));
}
