use parse::Membership::*;
use parse::Faction::*;
use std::collections::{BTreeSet, VecDeque};
// Unicode tables for character classes are defined in libunicode
use unicode::regex::{PERLD, PERLS, PERLW};

pub type CharSet = BTreeSet<Ast>;

trait Table {
    fn to_char_set(&self) -> CharSet;
}

impl Table for &'static [(char, char)] {
    fn to_char_set(&self) -> CharSet {
        let mut set = BTreeSet::new();

        for &(open, close) in self.iter() {
            set.insert(Ast::Range(open, close));
        }

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Faction {
    Capture,
    NonCapture,
}

// Abstract syntax tree
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Ast {
    Empty,
    Char(char),                     // abc123
    Class(VecDeque<Ast>),            // <[135] + [68\w]>
    Dot,                            // .
    Group(Vec<Ast>, Faction),       // [123] or (123) outside a `<>`
    Literal(Vec<char>),             // `'hello'` or `"hello"`
    Op(Op),
    // Unicode uses (open, close) pairs to denote range. A set of these is
    // more efficient than specifying every character. A set may include a
    // few dozen pairs instead of 100s/1000s.
    Range(char, char),              // (open, close) range for character sets
    Set(CharSet, Membership),      // [1..5] or [68\w] inside a `<>`
}

impl Ast {
    /*fn negate(&mut self) {
        *self = match *self {
            Ast::Set(set, membership) => Ast::Set(set, membership.negate()),
            ast => panic!("Negating `{:?}` is invalid.", ast),
        };
    }*/
    fn to_char_set(c: char) -> CharSet {
        let mut set = BTreeSet::new();
        set.insert(Ast::Char(c));

        set
    }
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
        let mut first = true;

        if self.chars.len() == 0 { panic!("An empty regex is not allowed.") }

        loop {
            if first { first = false; }
            else if !self.next() { break; } 
            let c = self.cur();

            if is_whitespace(c) { continue }
            else if is_alphanumeric(c) {
                vec.push(Ast::Char(c));
                continue;
            }

            match c {
                '\\' => {
                    let ast = self.parse_escape();
                    vec.push(ast);
                },
                '\'' | '"' => {
                    let ast = self.parse_literal();
                    vec.push(ast);
                },
                '<' => {
                    let ast = self.parse_class();
                    vec.push(ast);
                },
                '.' => vec.push(Ast::Dot),
                _ => panic!("`{:?}` is not valid here.", c),
            }
        };

        vec
    }
    // Parse the `< [123 a] + [4 \d] - [\w \d] >`
    fn parse_class(&mut self) -> Ast {
        // Classes will need to be merged later which requires collapsing from the
        // front so I'm using a deque (`<[abc] + [cde]>` collapses to `<[a...e]>`).
        let mut buffer = VecDeque::new();
        let mut closed = false; // Deliminator hasn't been closed yet.

        while self.next() {
            let c = self.cur();

            if is_whitespace(c) { continue; }

            match c {
                '-' => buffer.push_back(Ast::Op(Op::Difference)),
                '^' => buffer.push_back(Ast::Op(Op::SymmetricDifference)),
                '&' => buffer.push_back(Ast::Op(Op::Intersection)),
                '+' | '|' => buffer.push_back(Ast::Op(Op::Union)),
                '[' => buffer.push_back(self.parse_class_set()),
                '>' => {
                    closed = true;
                    break;
                },
                _   => panic!("`{:?}` is invalid inside `<>` and outside `[]`.", c),
            }
        }

        if !closed { panic!("A `<` must have a closing `>`."); }

        // Insert `Empty` in front if first character is a binary op.
        match buffer[0] {
            Ast::Op(Op::Difference) |
            Ast::Op(Op::SymmetricDifference) |
            Ast::Op(Op::Intersection) |
            Ast::Op(Op::Union) => buffer.push_front(Ast::Empty),
            _ => {},
        }

        // Insert `Empty` in back if last character is a binary op.
        match buffer[buffer.len()-1] {
            Ast::Op(Op::Difference) |
            Ast::Op(Op::SymmetricDifference) |
            Ast::Op(Op::Intersection) |
            Ast::Op(Op::Union) => buffer.push_back(Ast::Empty),
            _ => {},
        }

        Ast::Class(buffer)
    }
    // Inside a `<>`, parse the `[123 a]` or `[4 \d]`. Assume `[` is the first char.
    fn parse_class_set(&mut self) -> Ast {
        // Need a set but an ellipsis will require pulling the last element back off
        // the end. A set may not preserve order so a vec is used to build then
        // morphed into a set later.
        let mut vec = vec![];
        let mut closed = false; // Deliminator hasn't been closed yet.

        while self.next() {
            let c = self.cur();
            if is_whitespace(c) { continue; }

            match c {
                '\\' => vec.push(self.parse_escape()),
                ']'  => {
                    closed = true;
                    break;
                },
                '.' => {
                    if self.peek('.') {
                        self.next(); // Advance to second `.`
                        // Pull off the last `Ast` before the `..`
                        let before = match vec.pop() {
                            Some(Ast::Char(c)) => c,
                            Some(_) => panic!("`..` only operate on characters."),
                            None => panic!("`..` cannot be the first element in a character class."),
                        };

                        vec.push(self.parse_ellipsis(before));
                    } else { vec.push(Ast::Char(c)) }
                },
                c => vec.push(Ast::Char(c)),
            };
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
            if is_whitespace(b) { continue; }

            return Ast::Range(a, b);
        }

        panic!("An `..` must be followed by another char.");
    }
    // Parse the `\w`, `\d`, ... types
    fn parse_escape(&mut self) -> Ast {
        if !self.next() { panic!("A `\\` must be followed by another char."); }
        
        match self.cur() {
            'd' => Ast::Set(PERLD.to_char_set(), Inclusive),
            'D' => Ast::Set(PERLD.to_char_set(), Exclusive),
            'n' => Ast::Set(Ast::to_char_set('\n'), Inclusive),
            'N' => Ast::Set(Ast::to_char_set('\n'), Exclusive),
            't' => Ast::Set(Ast::to_char_set('\t'), Inclusive),
            'T' => Ast::Set(Ast::to_char_set('\t'), Exclusive),
            's' => Ast::Set(PERLS.to_char_set(), Inclusive),
            'S' => Ast::Set(PERLS.to_char_set(), Exclusive),
            'w' => Ast::Set(PERLW.to_char_set(), Inclusive),
            'W' => Ast::Set(PERLW.to_char_set(), Exclusive),
            c   => Ast::Set(Ast::to_char_set(c), Inclusive),
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
        if self.pos == 0 { return false }

        self.pos -= 1;

        true
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
    use super::Table;
    use super::CharSet;

    fn new_buf(vec: Vec<Ast>) -> VecDeque<Ast> {
        let buffer: VecDeque<Ast> = vec.into_iter().collect();

        buffer
    }
    fn new_set(vec: Vec<Ast>) -> CharSet {
        let set: CharSet = vec.into_iter().collect();

        set
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
        let set = new_set(vec![Char('a'), Char('b'), Char('c')]);
        // Buffer of ops and sets inside `<>`
        let buf = new_buf(vec![Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(buf)], parse(r"< [ abc ] >"));
    }
    #[test]
    fn char_class_with_ops() {
        // Set of chars inside `[]`
        let set_a = new_set(vec![Char('a')]);
        let set_b = new_set(vec![Char('b')]);
        // Buffer of ops and sets inside `<>`
        let buf = new_buf(vec![Set(set_a, Inclusive),
                               Op(Union),
                               Set(set_b, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(buf)], parse(r"< [ a ] + [ b ] >"));
    }
    #[test]
    fn char_class_pre_op() {
        // Set of chars inside `[]`
        let set = new_set(vec![Char('a')]);
        // Buffer of ops and sets inside `<>`. Pre-ops get an `Empty`
        // prepended so binary ops can be applied to them.
        let buf = new_buf(vec![Empty,
                               Op(Difference),
                               Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(buf)], parse(r"< - [ a ] >"));
    }
    #[test]
    fn char_class_post_op() {
        // Set of chars inside `[]`
        let set = new_set(vec![Char('a')]);
        // Buffer of ops and sets inside `<>`. Post-ops get an `Empty`
        // appended so binary ops can be applied to them.
        let buf = new_buf(vec![Set(set, Inclusive),
                               Op(Union),
                               Empty]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(buf)], parse(r"< [ a ] + >"));
    }
    #[test]
    fn char_class_ellipsis() {
        // Set of chars inside `[]`
        let set = new_set(vec![Range('a','d')]);
        // Buffer of ops and sets inside `<>`.
        let buf = new_buf(vec![Set(set, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(buf)], parse(r"< [ a .. d ] >"));
    }
    #[test]
    fn char_class_escape_chars() {
        // Set of chars inside `[]`. `\s` is a set itself inside the set `[]`.
        // Escaped character sets may be inclusive or exclusive and need to
        // keep their own parameters. So they are stored whole inside the outer
        // set to eventually be flattened into a single set after parsing.
        let set1 = new_set(vec![Set(PERLS.to_char_set(), Inclusive)]);
        // Buffer of ops and sets inside `<>`.
        let buf1 = new_buf(vec![Set(set1, Inclusive)]);
        assert_eq!(vec![Class(buf1)], parse(r"< [ \s ] >"));

        // Without the superset, these 2 would merge so they are stored whole
        // until later.
        let set2 = new_set(vec![Set(PERLS.to_char_set(), Inclusive),
                                Set(PERLS.to_char_set(), Exclusive)]);
        // Buffer of ops and sets inside `<>`.
        let buf2 = new_buf(vec![Set(set2, Inclusive)]);
        // A single class denoted by `<[]>`
        assert_eq!(vec![Class(buf2)], parse(r"< [ \s \S ] >"));
    }
    #[test]
    fn char_class_unicode() {
        // Set of chars inside `[]`
        let set = new_set(vec![Char('a'), Char('こ')]);
        // Buffer of ops and sets inside `<>`.
        let buf = new_buf(vec![Set(set, Inclusive)]);
        assert_eq!(vec![Class(buf)], parse(r"< [ a こ ] >"));
    }
    #[test]
    fn multi_char_class() {
        // Set of chars inside `[]`
        let set_a = new_set(vec![Char('a')]);
        let set_c = new_set(vec![Char('c')]);

        // Buffer of ops and sets inside `<>`
        let buf_a = new_buf(vec![Set(set_a, Inclusive)]);
        let buf_c = new_buf(vec![Set(set_c, Inclusive)]);

        // Multiple character classes
        assert_eq!(vec![Class(buf_a),
                        Char('b'),
                        Class(buf_c)], parse(r"<[ a ]> b <[ c ]>"));
    }
}
