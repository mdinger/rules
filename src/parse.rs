use parse::Membership::*;
use parse::Faction::*;
use range_set::{Range, Set};
use std::collections::VecDeque;
use std::{char, fmt};
use std::convert::Into;
use std::result;
// Unicode tables for character classes are defined in libunicode
use unicode::regex::{PERLD, PERLS, PERLW};

pub type Result<T> = result::Result<T, ParseError>;

#[derive(Debug)]
enum ParseError {
    ClassInvalid(char),
    ClassMustClose,
    ClassSetMustClose,
    EllipsisCloseNeedsEscape,
    EllipsisNotFirst,
    EllipsisNotLast,
    EllipsisOnlyChar,
    EmptyRegex,
    EscapeNotLast,
    Invalid(char),
    LiteralMustClose(char),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", match *self {
            ParseError::ClassInvalid(ref c) => 
                format!("`{}` is invalid inside `<>` and outside `[]`.", c),
            ParseError::ClassMustClose    => "A `<` must have a closing `>`.".to_owned(),
            ParseError::ClassSetMustClose => "A `[` must have a closing `]`.".to_owned(),
            ParseError::EllipsisCloseNeedsEscape =>
                "An `..` cannot be closed by an unescaped `]`".to_owned(),
            ParseError::EllipsisNotFirst  => "`..` cannot be the first element in a character class.".to_owned(),
            ParseError::EllipsisNotLast   => "An `..` must be followed by another char.".to_owned(),
            ParseError::EllipsisOnlyChar  => "`..` only operate on characters.".to_owned(),
            ParseError::EmptyRegex        => "An empty regex is not allowed.".to_owned(),
            ParseError::EscapeNotLast     => "A `\\` must be followed by another char.".to_owned(),
            ParseError::Invalid(ref c)    => format!("`{}` is not valid here.", c),
            ParseError::LiteralMustClose(ref c) => 
                format!("A literal must have an opening and closing `{}`.", c),
        })
    }
}

impl Into<Set> for &'static [(char, char)] {
    fn into(self) -> Set {
        let mut set = Set::new();

        for &(open, close) in self {
            set.insert(Range(open, close));
        }

        set
    }
}

impl Into<Set> for char {
    fn into(self) -> Set {
        let mut set = Set::new();
        set.insert(Range(self, self));

        set
    }
}

// A set may be composed of Chars and Ranges but other types
// have no meaning here. These are the only applicable types.
impl Into<Set> for Vec<Ast> {
    fn into(self) -> Set {
        let mut set = Set::new();

        for ast in self {
            match ast {
                Ast::Char(c)      => set.insert(Range(c, c)),
                Ast::Range(range) => set.insert(range),
                x => { println!("x: {:?}", x);
                       unreachable!() },
            }
        }

        set
    }
}

impl Into<Ast> for Vec<Ast> {
    fn into(self) -> Ast {
        let (mut inclusive, mut exclusive) = (Set::new(), Set::new());

        for ast in self {
            match ast {
                Ast::Char(c) => inclusive.insert(Range(c, c)),
                Ast::Range(range) => inclusive.insert(range),
                Ast::Set(set, Inclusive) => inclusive = inclusive.union(&set),
                Ast::Set(set, Exclusive) => exclusive = exclusive.union(&set),
                _ => unreachable!(),
            }
        }

        if exclusive.is_empty() { Ast::Set(inclusive, Inclusive) }
        else { Op::Union.apply(Ast::Set(inclusive, Inclusive),
                               Ast::Set(exclusive, Exclusive)) }
    }
}

pub trait NextPrev {
    fn next(&self) -> Self;
    fn prev(&self) -> Self;
}

impl NextPrev for char {
    fn next(&self) -> Self { char::from_u32(*self as u32 + 1).unwrap() }
    fn prev(&self) -> Self { char::from_u32(*self as u32 - 1).unwrap() }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Membership {
    Exclusive,
    Inclusive,
}

impl Membership {
    fn negate(self) -> Self {
        match self {
            Exclusive  => Inclusive,
            Inclusive  => Exclusive,
        }
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

impl Op {
    // Apply set operations. Must have `Char` removed beforehand. Subsets should be
    // removed beforehand. This will remove new subsets before exiting.
    pub fn apply(&self, left: Ast, right: Ast) -> Ast {

        match *self {
            Op::Difference          => self.difference(left, right),
            Op::SymmetricDifference => self.symmetric_difference(left, right),
            Op::Intersection        => self.intersection(left, right),
            Op::Union               => self.union(left, right),
            _ => unimplemented!(),
        }
    }
    // Apply set difference.
    fn difference(&self, left: Ast, right: Ast) -> Ast {
        match (left, right) {
            (Ast::Empty, right) => right.negate(),
            (left, Ast::Empty)  => left,
            (Ast::Set(lset, lmem), Ast::Set(rset, rmem)) => {
                match (lmem, rmem) {
                    (l, r) if l == r => Ast::Set(lset.difference(&rset), l),
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
    // Apply symmetric set difference.
    fn symmetric_difference(&self, left: Ast, right: Ast) -> Ast {
        match (left, right) {
            (Ast::Empty, right) => right,
            (left, Ast::Empty)  => left,
            (Ast::Set(lset, lmembership), Ast::Set(rset, rmembership)) => {
                if lmembership == rmembership {
                    Ast::Set(lset.symmetric_difference(&rset), lmembership)
                } else { unimplemented!() }
            },
            _ => unimplemented!(),
        }
    }
    // Apply set intersection.
    fn intersection(&self, left: Ast, right: Ast) -> Ast {
        match (left, right) {
            (Ast::Empty, _) |
            (_, Ast::Empty)  => Ast::Empty,
            (Ast::Set(lset, lmem), Ast::Set(rset, rmem)) => {
                match (lmem, rmem) {
                    (l, r) if l == r => Ast::Set(lset.intersection(&rset), l),
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
    // Apply set union.
    fn union(&self, left: Ast, right: Ast) -> Ast {
        match (left, right) {
            (Ast::Empty, right) => right,
            (left , Ast::Empty) => left,
            (Ast::Set(lset, lmembership), Ast::Set(rset, rmembership)) => {
                // Unifying sets with opposite membership isn't obvious. If
                // -3 is 3 Exclusive and 3 is Inclusive then `-3 + 3` is a
                // union which is identical to `-(3 - 3)` = `-()`. Similarly,
                // `-1 + 7` = `-(1 - 7)` = `-1`.
                match (lmembership, rmembership) {
                    (x, y) if x == y   => Ast::Set(lset.union(&rset), x),
                    (x @ Exclusive, _) => Ast::Set(lset.difference(&rset), x),
                    (Inclusive, y)     => Ast::Set(rset.difference(&lset), y),
                }
            },
            _ => unreachable!(),
        }
    }
}

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
    Range(Range),                   // (open, close) range for character sets
    Set(Set, Membership),           // [1..5 \d] inside a `<>`
}

impl Ast {
    fn negate(self) -> Self {
        match self {
            Ast::Set(set, membership) => Ast::Set(set, membership.negate()),
            _ => unreachable!(),
        }
    }
}

pub fn parse(s: &str) -> Result<Vec<Ast>> {
    Parser { chars: s.chars().collect(),
             pos: 0,
    }.parse()
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn cur(&self) -> char { self.chars[self.pos] }
    // True if next finds another char.
    fn next(&mut self) -> bool {
        self.pos += 1;

        self.pos != self.chars.len()
    }
    fn parse(&mut self) -> Result<Vec<Ast>> {
        let mut vec = vec![];

        if self.chars.len() == 0 { return Err(ParseError::EmptyRegex) }

        loop {
            let c = self.cur();

            if c.is_alphanumeric() || c == '_' { vec.push(Ast::Char(c)) }
            else if !c.is_whitespace() {
                vec.push(try!(match c {
                    '\\'       => self.parse_escape_set(),
                    '\'' | '"' => self.parse_literal(),
                    '<'        => self.parse_class(),
                    '.'        => Ok(Ast::Dot),
                    _          => Err(ParseError::Invalid(c)),
                }));
            }

            if !self.next() { break }
        }

        Ok(vec)
    }
    // Parse the `< [123 a] + [4 \d] - [\w \d] >`
    fn parse_class(&mut self) -> Result<Ast> {
        // Classes will need to be merged later which requires collapsing from the
        // front so I'm using a deque (`<[abc] + [cde]>` collapses to `<[a...e]>`).
        let mut deque = VecDeque::new();
        let mut closed = false; // Deliminator hasn't been closed yet.

        while self.next() {
            let c = self.cur();

            if c == '>' {
                closed = true;
                break;
            } else if !c.is_whitespace() {
                deque.push_back(try!(match c {
                    '-'       => Ok(Ast::Op(Op::Difference)),
                    '^'       => Ok(Ast::Op(Op::SymmetricDifference)),
                    '&'       => Ok(Ast::Op(Op::Intersection)),
                    '+' | '|' => Ok(Ast::Op(Op::Union)),
                    '['       => self.parse_class_set(),
                    _         => Err(ParseError::ClassInvalid(c)),
                }));
            }
        }

        if !closed { return Err(ParseError::ClassMustClose) }

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

        Ok(Ast::Class(deque))
    }
    // Inside a `<>`, parse the `[123 a]` or `[4 \d]`. Assume `[` is the first char.
    fn parse_class_set(&mut self) -> Result<Ast> {
        // Need a set but an ellipsis will require pulling the last element back off
        // the end. A set may not preserve order so a vec is used to build then
        // morphed into a set later.
        let mut vec = vec![];
        let mut closed = false; // Deliminator hasn't been closed yet.

        while !closed && self.next() {
            let c = self.cur();

            if c == ']' { closed = true } else if !c.is_whitespace() {
                let ast = try!(match c {
                    '\\' => self.parse_escape_set(),
                    '.'  => {
                        if self.peek('.') {
                            self.next(); // Advance to second `.`
                            // Pull off the last `Ast` before the `..`
                            let before = try!(match vec.pop() {
                                Some(Ast::Char(c)) => Ok(c),
                                Some(_) => Err(ParseError::EllipsisOnlyChar),
                                None    => Err(ParseError::EllipsisNotFirst),
                            });

                            self.parse_ellipsis(before)
                        } else { Ok(Ast::Char(c)) }
                    },
                    c    => Ok(Ast::Char(c)),
                });

                vec.push(ast);
            }
        }

        if !closed { return Err(ParseError::ClassSetMustClose) }

        Ok(vec.into())
    }
    // The `a .. b` notation has been parsed. Determine `b` and return a `Range`
    // from `a` to `b`.
    fn parse_ellipsis(&mut self, a: char) -> Result<Ast> {
        while self.next() {
            let b = self.cur();
            if !b.is_whitespace() {
                return match b {
                    ']'  => Err(ParseError::EllipsisCloseNeedsEscape),
                    '\\' => self.parse_escape().map(|c| Ast::Range(Range(a, c))),
                    _    => Ok(Ast::Range(Range(a, b))),
                };
            }
        }
        
        Err(ParseError::EllipsisNotLast)
    }
    // Return the next character which follows a `\`.
    fn parse_escape(&mut self) -> Result<char> {
        if !self.next() { return Err(ParseError::EscapeNotLast) }

        Ok(self.cur())
    }
    // Parse the `\w`, `\d`, ... types
    fn parse_escape_set(&mut self) -> Result<Ast> {
        self.parse_escape()
            .map(|c| match c {
            'd' => Ast::Set(PERLD.into(), Inclusive),
            'D' => Ast::Set(PERLD.into(), Exclusive),
            'n' => Ast::Set('\n'.into(), Inclusive),
            'N' => Ast::Set('\n'.into(), Exclusive),
            't' => Ast::Set('\t'.into(), Inclusive),
            'T' => Ast::Set('\t'.into(), Exclusive),
            's' => Ast::Set(PERLS.into(), Inclusive),
            'S' => Ast::Set(PERLS.into(), Exclusive),
            'w' => Ast::Set(PERLW.into(), Inclusive),
            'W' => Ast::Set(PERLW.into(), Exclusive),
            c   => Ast::Char(c),
        })
    }
    // Parse the `'hello world'` and `"testing_this"`
    fn parse_literal(&mut self) -> Result<Ast> {
        let close = self.cur();
        let mut vec = vec![];

        while self.next() {
            let c = self.cur();
            if c == close { return Ok(Ast::Literal(vec)) }

            vec.push(c);
        }

        Err(ParseError::LiteralMustClose(close))
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
