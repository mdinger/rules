use rules::unicode::regex::PERLS;
use rules::parse::Ast::*;
use rules::parse::Membership::*;
use rules::parse::ToCharSet;

use super::parse;

#[test]
fn text() {
    assert_eq!(vec![Char('f'), Char('r'), Char('e'), Char('d')], parse(r"fred"));
    assert_eq!(vec![Char('t'), Char('_'), Char('d')], parse(r"t_d"));
}
#[test]
fn ignore_whitespace() {
    assert_eq!(vec![Char('f'), Char('r'), Char('e'), Char('d')], parse(r" f r e d "));
    assert_eq!(vec![Char('b'), Char('o'), Char('f'), Char('l'), Char('e'), Char('x')],
               parse(r" bo
    flex "));
}
#[test]
fn dot() {
    assert_eq!(vec![Char('f'), Char('r'), Char('e'), Dot], parse(r" fre. "));
    assert_eq!(vec![Char('t'), Dot, Dot, Char('e')], parse(r" t..e "));
}
#[test]
fn unicode() {
    assert_eq!(vec![Char('こ'), Char('ん'), Char('に')], parse(r"こんに"));
}
#[test]
fn escapes() {
    assert_eq!(vec![Set(PERLS.to_char_set(), Inclusive)], parse(r"\s"));
    assert_eq!(vec![Set(PERLS.to_char_set(), Exclusive)], parse(r"\S"));
}
