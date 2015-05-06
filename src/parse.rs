use parse::Membership::*;
use parse::Faction::*;
use std::collections::{BTreeSet, VecDeque};
// Unicode tables for character classes are defined in libunicode
use unicode::regex::{PERLD, PERLS, PERLW};

pub type CharSet = BTreeSet<Ast>;

trait ToCharSet {
    fn to_char_set(&self) -> CharSet;
}

impl ToCharSet for &'static [(char, char)] {
    fn to_char_set(&self) -> CharSet {
        let mut set = BTreeSet::new();

        for &(open, close) in *self {
            set.insert(Ast::Range(open, close));
        }

        set
    }
}

impl ToCharSet for char {
    fn to_char_set(&self) -> CharSet {
        let mut set = BTreeSet::new();
        set.insert(Ast::Char(*self));

        set
    }
}

impl ToCharSet for Vec<Ast> {
    fn to_char_set(&self) -> CharSet {
        let vec = (*self).clone();
        let set: CharSet = vec.into_iter().collect();

        set
    }
}

enum Unicode {
    Digit,
    Whitespace,
    Word,
}

impl Unicode {
    fn to_set(&self) -> Ast {
        Ast::Set(match *self {
            Unicode::Digit      => PERLD.to_char_set(),
            Unicode::Whitespace => PERLS.to_char_set(),
            Unicode::Word       => PERLW.to_char_set(),
        },
        Inclusive)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Membership {
    Exclusive,
    Inclusive,
}

impl Membership {
    fn negate(&mut self) {
        *self = match *self {
            Exclusive  => Inclusive,
            Inclusive  => Exclusive,
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Op {
    Ellipsis,               // ..
    Difference,             // -
    SymmetricDifference,    // ^
    Intersection,           // &
    Union,                  // + or |
}

/*impl Op {
    fn difference(class: &mut VecDeque<Ast>) {
        // `class[1]` is the op which called this function.
        let difference = match (class[0], class[2]) {
            (Ast::Empty, group) => group.negate(),
            (group, Ast::Empty) => group,
            (Ast::Set(first_vec, first_membership), Ast::Empty) => group,
            (group, Ast::Empty) => group,
        };
    }
}*/

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Faction {
    Capture,
    NonCapture,
}

// Abstract syntax tree
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Ast {
    Empty,
    Char(char),                     // abc123
    Class(VecDeque<Ast>),           // <[135] + [68\w]>
    Dot,                            // .
    Group(Vec<Ast>, Faction),       // [123] or (123) outside a `<>`
    Literal(Vec<char>),             // `'hello'` or `"hello"`
    Op(Op),
    // Unicode uses (open, close) pairs to denote range. A set of these is
    // more efficient than specifying every character. A set may include a
    // few dozen pairs instead of 100s/1000s.
    Range(char, char),              // (open, close) range for character sets
    Set(CharSet, Membership),       // [1..5] or [68\w] inside a `<>`
}

impl Ast {
    /*fn negate(&mut self) {
        *self = match *self {
            Ast::Set(set, membership) => Ast::Set(set, membership.negate()),
            ast => panic!("Negating `{:?}` is invalid.", ast),
        };
    }*/
}

pub fn parse(s: &str) -> Vec<Ast> {
    Parser { chars: s.chars().collect(),
             pos: 0,
    }.parse()
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn cur(&self)  -> char { self.chars[self.pos] }
    // True if next finds another char.
    fn next(&mut self) -> bool {
        self.pos += 1;

        self.pos != self.chars.len()
    }
    fn parse(&mut self) -> Vec<Ast> {
        let mut vec = vec![];

        if self.chars.len() == 0 { panic!("An empty regex is not allowed.") }

        loop {
            let c = self.cur();

            if is_alphanumeric(c) { vec.push(Ast::Char(c)) }
            else if !is_whitespace(c) {
                vec.push(match c {
                    '\\'       => self.parse_escape(),
                    '\'' | '"' => self.parse_literal(),
                    '<'        => self.parse_class(),
                    '.'        => Ast::Dot,
                    _          => panic!("`{}` is not valid here.", c),
                });
            }

            if !self.next() { break }
        }

        vec
    }
    // Parse the `< [123 a] + [4 \d] - [\w \d] >`
    fn parse_class(&mut self) -> Ast {
        // Classes will need to be merged later which requires collapsing from the
        // front so I'm using a deque (`<[abc] + [cde]>` collapses to `<[a...e]>`).
        let mut deque = VecDeque::new();
        let mut closed = false; // Deliminator hasn't been closed yet.

        while self.next() {
            let c = self.cur();

            if c == '>' {
                closed = true;
                break;
            } else if !is_whitespace(c) {
                deque.push_back(match c {
                    '-'       => Ast::Op(Op::Difference),
                    '^'       => Ast::Op(Op::SymmetricDifference),
                    '&'       => Ast::Op(Op::Intersection),
                    '+' | '|' => Ast::Op(Op::Union),
                    '['       => self.parse_class_set(),
                    _         => panic!("`{:?}` is invalid inside `<>` and outside `[]`.", c),
                });
            }
        }

        if !closed { panic!("A `<` must have a closing `>`."); }

        // Insert `Empty` in front if first character is a binary op.
        match deque[0] {
            Ast::Op(Op::Difference) |
            Ast::Op(Op::SymmetricDifference) |
            Ast::Op(Op::Intersection) |
            Ast::Op(Op::Union) => deque.push_front(Ast::Empty),
            _ => {},
        }

        // Insert `Empty` in back if last character is a binary op.
        match deque[deque.len()-1] {
            Ast::Op(Op::Difference) |
            Ast::Op(Op::SymmetricDifference) |
            Ast::Op(Op::Intersection) |
            Ast::Op(Op::Union) => deque.push_back(Ast::Empty),
            _ => {},
        }

        Ast::Class(deque)
    }
    // Inside a `<>`, parse the `[123 a]` or `[4 \d]`. Assume `[` is the first char.
    fn parse_class_set(&mut self) -> Ast {
        // Need a set but an ellipsis will require pulling the last element back off
        // the end. A set may not preserve order so a vec is used to build then
        // morphed into a set later.
        let mut vec = vec![];
        let mut closed = false; // Deliminator hasn't been closed yet.

        while !closed && self.next() {
            let c = self.cur();

            if c == ']' { closed = true } else if !is_whitespace(c) {
                let ast = match c {
                    '\\' => self.parse_escape(),
                    '.'  => {
                        if self.peek('.') {
                            self.next(); // Advance to second `.`
                            // Pull off the last `Ast` before the `..`
                            let before = match vec.pop() {
                                Some(Ast::Char(c)) => c,
                                Some(_) => panic!("`..` only operate on characters."),
                                None => panic!("`..` cannot be the first element in a character class."),
                            };

                            self.parse_ellipsis(before)
                        } else { Ast::Char(c) }
                    },
                    c    => Ast::Char(c),
                };

                vec.push(ast);
            }
        }

        if !closed { panic!("A `[` must have a closing `]`.") }

        let set: CharSet = vec.into_iter().collect();

        Ast::Set(set, Inclusive)
    }
    // The `a .. b` notation has been parsed. Determine `b` and return an
    // inclusive `Set` from `a` to `b`.
    fn parse_ellipsis(&mut self, a: char) -> Ast {
        while self.next() {
            let b = self.cur();
            if !is_whitespace(b) { return Ast::Range(a, b) }
        }

        panic!("An `..` must be followed by another char.");
    }
    // Parse the `\w`, `\d`, ... types
    fn parse_escape(&mut self) -> Ast {
        if !self.next() { panic!("A `\\` must be followed by another char."); }

        match self.cur() {
            'd' => Ast::Set(PERLD.to_char_set(), Inclusive),
            'D' => Ast::Set(PERLD.to_char_set(), Exclusive),
            'n' => Ast::Set('\n'.to_char_set(), Inclusive),
            'N' => Ast::Set('\n'.to_char_set(), Exclusive),
            't' => Ast::Set('\t'.to_char_set(), Inclusive),
            'T' => Ast::Set('\t'.to_char_set(), Exclusive),
            's' => Ast::Set(PERLS.to_char_set(), Inclusive),
            'S' => Ast::Set(PERLS.to_char_set(), Exclusive),
            'w' => Ast::Set(PERLW.to_char_set(), Inclusive),
            'W' => Ast::Set(PERLW.to_char_set(), Exclusive),
            c   => Ast::Set(c.to_char_set(), Inclusive),
        }
    }
    // Parse the `'hello world'` and `"testing_this"`
    fn parse_literal(&mut self) -> Ast {
        let close = self.cur();
        let mut vec = vec![];

        while self.next() {
            let c = self.cur();
            if c == close { return Ast::Literal(vec) }

            vec.push(c);
        }

        panic!("A literal must have an opening and closing `{:?}`.");
    }
    // Check if next character matches `needle`. Doesn't modify pos.
    fn peek(&mut self, needle: char) -> bool {
        let mut ret = false;

        if self.next() &&
           self.cur() == needle { ret = true; }

        // `self.next()` always increments pos even if it fails to find an element. So decrement.
        self.prev();

        ret
    }
    // True if prev finds another char.
    fn prev(&mut self) -> bool {
        if self.pos == 0 { false } else {
            self.pos -= 1;

            true
        }
    }
}

// See if unicode container contains character `c`
fn contains(container: &'static [(char, char)], c: char) -> bool {
    for &(open, close) in container {
        if open <= c && c <= close { return true; }
    }

    false
}
fn is_alphanumeric(c: char) -> bool {
    contains(PERLD, c) || contains(PERLW, c)
}
fn is_whitespace(c: char) -> bool {
    contains(PERLS, c)
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use unicode::regex::PERLS;
    use super::Ast;
    use super::Ast::*;
    use super::parse;
    use super::Membership::*;
    use super::Op::*;
    use super::ToCharSet;

    fn new_deque(vec: Vec<Ast>) -> VecDeque<Ast> {
        let deque: VecDeque<Ast> = vec.into_iter().collect();

        deque
    }

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
    #[test]
    fn char_class() {
        // Set of chars inside `[]`
        let set = vec![Char('a'), Char('b'), Char('c')].to_char_set();
        // Deque of ops and sets inside `<>`
        let deque = new_deque(vec![Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(deque)], parse(r"< [ abc ] >"));
    }
    #[test]
    fn char_class_with_ops() {
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
    fn char_class_pre_op() {
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
    fn char_class_post_op() {
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
    fn char_class_ellipsis() {
        // Set of chars inside `[]`
        let set = vec![Range('a','d')].to_char_set();
        // Deque of ops and sets inside `<>`.
        let deque = new_deque(vec![Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(deque)], parse(r"< [ a .. d ] >"));
    }
    #[test]
    fn char_class_escape_chars() {
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
    fn char_class_unicode() {
        // Set of chars inside `[]`
        let set = vec![Char('a'), Char('こ')].to_char_set();
        // Deque of ops and sets inside `<>`.
        let deque = new_deque(vec![Set(set, Inclusive)]);
        assert_eq!(vec![Class(deque)], parse(r"< [ a こ ] >"));
    }
    #[test]
    fn multi_char_class() {
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
}
