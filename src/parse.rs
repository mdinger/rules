use parse::Membership::*;
use parse::Faction::*;
use std::collections::{BTreeSet, VecDeque};
use std::{char, fmt};
use std::result;
// Unicode tables for character classes are defined in libunicode
use unicode::regex::{PERLD, PERLS, PERLW};

pub type CharSet = BTreeSet<Ast>;
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

pub trait ToCharSet {
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
        let vec = self.clone();
        let set: CharSet = vec.into_iter().collect();

        set
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

        let applied = match *self {
            Op::Difference          => self.difference(left, right),
            Op::SymmetricDifference => self.symmetric_difference(left, right),
            Op::Intersection        => self.intersection(left, right),
            Op::Union               => self.union(left, right),
            _ => unimplemented!(),
        };

        applied.remove_subsets()
    }
    // Set ranges can be overlapping or non-overlapping. This is problematic for matching
    // because very similar sets are considered distinct meaning set operations will
    // not function properly: range(2,3) + range(2,4) = { range(2,3), range(2,4) }.
    // This modifies the sets so they don't partially overlap. This will allow
    // inner sets to be removed later.
    fn align(&self, left: CharSet, right: CharSet) -> (CharSet, CharSet) {
        let mut lset = BTreeSet::new();
        let mut rset = BTreeSet::new();

        for l in &left {
            for r in &right {
                let (l_range, r_range) = match (l, r) {
                    (&Ast::Range(min1, max1), &Ast::Range(min2, max2)) => {
                        // First overlaps at the beginning.
                        let l = if min1 < min2 && max1 >= min2 { vec![Ast::Range(min1, min2.prev()),
                                                                      Ast::Range(min2, max1)] }
                        // First overlaps at the end.
                        else if min1 <= max2 && max1 > max2 { vec![Ast::Range(min1, max2),
                                                                   Ast::Range(max2.next(), max1)] }
                        // Complete overlap or no overlap.
                        else { vec![Ast::Range(min1, max1)] };

                        (l, vec![Ast::Range(min2, max2)])
                    },
                    _ => unreachable!(),
                };

                for i in l_range { lset.insert(i); }
                for j in r_range { rset.insert(j); }
            }
        }

        (lset, rset)
    }
    // Apply set difference.
    fn difference(&self, left: Ast, right: Ast) -> Ast {
        match (left, right) {
            (Ast::Empty, right) => right.negate(),
            (left, Ast::Empty)  => left,
            (Ast::Set(lset, lmem), Ast::Set(rset, rmem)) => {
                match (lmem, rmem) {
                    (l, r) if l == r => Ast::Set(lset.difference(&rset)
                                                     .cloned()
                                                     .collect(), l),
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
                    Ast::Set(lset.symmetric_difference(&rset)
                                 .cloned()
                                 .collect(), lmembership)
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
                    (l, r) if l == r => Ast::Set(lset.intersection(&rset)
                                                     .cloned()
                                                     .collect(), l),
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
                let (lset, rset) = self.align(lset, rset);
                // Unifying sets with opposite membership isn't obvious. If
                // -3 is 3 Exclusive and 3 is Inclusive then `-3 + 3` is a
                // union which is identical to `-(3 - 3)` = `-()`. Similarly,
                // `-1 + 7` = `-(1 - 7)` = `-1`.
                //
                // However, a `Range(x,y)` may appear inside a `Range(z,t)`
                // which a straight union wouldn't realize. Some manual prep
                // must be done.
                match (lmembership, rmembership) {
                    (lmem, rmem) if lmem == rmem => Ast::Set(lset.union(&rset)
                                                                 .cloned()
                                                                 .collect(), lmem),
                    (lmem @ Exclusive, _) => Ast::Set(lset.difference(&rset)
                                                          .cloned()
                                                          .collect(), lmem),
                    (Inclusive, rmem)     => Ast::Set(rset.difference(&lset)
                                                          .cloned()
                                                          .collect(), rmem),
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
    Range(char, char),              // (open, close) range for character sets
    Set(CharSet, Membership),       // [1..5] or [68\w] inside a `<>`
}

impl Ast {
    // While a class is being collapsed, a Set may include Sets inside it with
    // varying memberships. This flattens a Set into one single layer, applying
    // a union to sets when their membership varies.
    pub fn flatten(self) -> Self {
        let mut inclusive: CharSet = BTreeSet::new();
        let mut exclusive: CharSet = BTreeSet::new();

        if let Ast::Set(set_outer, membership_outer) = self {
            for i in set_outer {
                if let Ast::Set(set_inner, membership_inner) = i {
                    for j in set_inner {
                        if membership_inner == Inclusive { inclusive.insert(j); }
                        else { exclusive.insert(j); }
                    }
                } else {
                    if membership_outer == Inclusive { inclusive.insert(i); }
                    else { exclusive.insert(i); }
                }
            }

            // A union applied to an empty exclusion would include everything.
            // Check for empty sets to handle manually.
            //
            // They may still have `Char(c)` inside. Convert to `Range(c,c)` for
            // easier processing. Remove subsets before applying an op.
            //
            // Subset may be of form: `<[0..3 1..3]>` and a union should be applied.
            if exclusive.is_empty() { Ast::Set(inclusive, Inclusive).strip_char()
                                                                    .remove_subsets() }
            else { Op::Union.apply(Ast::Set(inclusive, Inclusive).strip_char()
                                                                 .remove_subsets(),
                                   Ast::Set(exclusive, Exclusive).strip_char()
                                                                 .remove_subsets()) }
        } else { self }
    }
    fn negate(self) -> Self {
        match self {
            Ast::Set(set, membership) => Ast::Set(set, membership.negate()),
            _ => unreachable!(),
        }
    }
    // A set such as `2..3 + \d` would result in 2 separate ranges being stored:
    // Range(2,3) and the other which is it's superset. The reason is because
    // they have different values, they are considered distinct. This removes
    // subsets.
    //
    // BTreeSet seems to be sorted by the first element which is good. This
    // doesn't have to be O(2).
    pub fn remove_subsets(self) -> Self {
        let mut ret: CharSet = BTreeSet::new();
        let mut first = true;
        // Filler to appease the compiler.
        let mut previous = Ast::Empty;

        match self {
            Ast::Set(set, membership) => {
                for current in set {
                    if first {
                        first = false;
                        previous = current;
                    } else {
                        previous = previous.superset(&current)
                                           .map_or_else( || { ret.insert(previous);
                                                              current },
                                                         |x| x);
                    }
                }

                // If passed empty set, the above loop will never run.
                if !first { ret.insert(previous); }
                Ast::Set(ret, membership)
            },
            ret => ret,
        }
    }
    // Dealing with all variations of `Char(c)` and `Range(a, b)` is complicated.
    // So all `Char` are stripped out of character classes for easier simplification.
    pub fn strip_char(self) -> Self {
        let mut ret = BTreeSet::new();

        if let Ast::Set(set, membership) = self {
            for i in set {
                ret.insert(match i {
                    Ast::Char(c) => Ast::Range(c, c),
                    x => x,
                });
            }

            Ast::Set(ret, membership)
        } else { self }
    }
    // Change all `Range(x, x)` back to `Char(x)` since we've finished set ops.
    pub fn strip_double_range(self) -> Self {
        let mut ret = BTreeSet::new();

        if let Ast::Set(set, membership) = self {
            for i in set {
                ret.insert(match i {
                    Ast::Range(x, y) if x == y => Ast::Char(x),
                    x => x,
                });
            }

            Ast::Set(ret, membership)
        } else { self }
    }
 
    // Returns the superset of the two or None.
    fn superset(&self, r: &Ast) -> Option<Self> {
        match (self, r) {
            (&Ast::Range(min1, max1), &Ast::Range(min2, max2)) => {
                if      min1 <= min2 && max1 >= max2 { Some(Ast::Range(min1, max1)) }
                else if min2 <= min1 && max2 >= max1 { Some(Ast::Range(min2, max2)) }
                else { None }
            },
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
    fn cur(&self)  -> char { self.chars[self.pos] }
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

            if is_alphanumeric(c) { vec.push(Ast::Char(c)) }
            else if !is_whitespace(c) {
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
            } else if !is_whitespace(c) {
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

            if c == ']' { closed = true } else if !is_whitespace(c) {
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

        let set: CharSet = vec.into_iter().collect();

        Ok(Ast::Set(set, Inclusive))
    }
    // The `a .. b` notation has been parsed. Determine `b` and return an
    // inclusive `Set` from `a` to `b`.
    fn parse_ellipsis(&mut self, a: char) -> Result<Ast> {
        while self.next() {
            let b = self.cur();
            if !is_whitespace(b) {
                return match b {
                    ']'  => Err(ParseError::EllipsisCloseNeedsEscape),
                    '\\' => self.parse_escape().map(|c| Ast::Range(a, c)),
                    _    => Ok(Ast::Range(a, b)),
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

// See if unicode container contains character `c`
fn contains(container: &'static [(char, char)], c: char) -> bool {
    container.iter().any(|&(open, close)| open <= c && c <= close )
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
    use super::Membership::*;
    use super::Op::*;
    use super::ToCharSet;

    fn new_deque(vec: Vec<Ast>) -> VecDeque<Ast> {
        let deque: VecDeque<Ast> = vec.into_iter().collect();

        deque
    }
    fn parse(s: &str) -> Vec<Ast> {
        super::parse(s).unwrap()
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
