#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(default_type_params)]
#![allow(unused_variables)]
#![allow(dead_code)]

//! # Rules
//! Rules uses regular expressions to do pattern matching using syntax
//! based upon the Perl 6 regex grammar. The Perl 6 grammar has been heavily
//! revised from Perl 5 and should not be equated with it. This may look
//! nothing like any regex you have seen before.
//!
//! # Syntax
//!
//! Currently, this is designed for ASCII and may not behave properly
//! with Unicode.
//!
//! Whitespace is generally ignored to so that a regex can be more readable
//! and less dense.
//!
//! ```rust
//! r"fred"    // Normal way
//! r"f r e d" // Completely equivalent
//! // Will match `apples_oranges` or any other deliminator
//! r"apples . oranges"
//! ```
//!
//! # Set operators
//!
//! These operators can be applied to groups which will be analyzed later:
//!
//! ```plain
//! +       Union                // [123] + [345] = [12345]
//! |       Union                // Same
//! &       Intersection         // [123] & [345] = [3]
//! -       Complement           // [123] - [345] = [1]
//! ^       Symmetric difference // [123] ^ [345] = [1245]
//! ```
//!
//! # Character classes
//!
//! ## Default character classes
//!
//! Character | Matches               | Inverse
//! ----------|-----------------------|--------
//! `.`       | Any character         | N/A
//! `\d`      | Digit                 | `\D`
//! `\h`      | Horizontal whitespace | `\H`
//! `\n`      | Newline               | `\N`
//! `\s`      | Any whitespace        | `\S`
//! `\t`      | Tab                   | `\T`
//! `\w`      | Alphanumeric or `_`   | `\W`
//!
//! ## Custom character classes
//!
//! Characters inside a set of `<[ ]>` form a custom character
//! class:
//!
//! ```plain
//! // Matches `a` or `b` or `c`
//! <[ a b c ]>
//!
//! // `..` expresses a range so this matches
//! // from `a` to `g` or a digit
//! <[ a .. g \d ]>
//!
//! // The `[]` bind the sets together into (non-capturing)
//! // groups so set operators can be used.
//! <[0-9] - [13579]> // Matches an even number
//! <\d - [13579]>    // Same
//! <[0-6] ^ [5-9]>   // Matches a `5` or `6`
//! ```
//!
//! # Comments
//!
//! Comments are allowed inside a regex.
//!
//! ```rust
//! // This matches `myregex`
//! r"my // This is a comment which goes to the end of the line
//! regex"
//! ```

use std::fmt;

#[macro_export]
macro_rules! matches(
    ($expression: expr, $($pattern:pat)|+) => (
        matches!($expression, $($pattern)|+ if true)
    );
    ($expression: expr, $($pattern:pat)|+ if $guard: expr) => (
        match $expression {
            $($pattern)|+ => $guard,
            _ => false
        }
    );
);

enum Mode {
    Match,
    Scan,
}

struct CharSet {
    chars: String,
    // If true include the exact contents. Otherwise, include
    // everything *not* inside `chars`.
    include: bool,
}

impl CharSet {
    fn new(meta: Meta) -> CharSet {
        match meta {
            Meta::Char(c)           => CharSet { chars: String::from_char(1, c), include: true },
            Meta::Digit(test)       => CharSet { chars: "0123456789".to_string(), include: test },
            Meta::Dot               => CharSet { chars: "".to_string(), include: false },
            Meta::Newline(test)     => CharSet { chars: "\n".to_string(), include: test },
            Meta::Tab(test)         => CharSet { chars: "\t".to_string(), include: test },
            Meta::Whitespace(test)  => CharSet { chars: " \t\n".to_string(), include: test },
            Meta::HWhitespace(test) => CharSet { chars: " \t".to_string(), include: test },
            Meta::Word(test)        => CharSet { chars:
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_".to_string(),
                include: test },
        }
    }
}

// Types
enum Ty {
    Group(Group),
    Meta(Meta),
    Op(Op),
}

impl fmt::Show for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ty::Group(ref ty) => write!(f, "{}", ty),
            Ty::Meta(ref ty)  => write!(f, "{}", ty),
            Ty::Op(ref ty)    => write!(f, "{}", ty),
        }
    }
}

enum Op {
    Ellipsis,                  // ..
    SetDifference,             // -
    SetSymmetricDifference,    // ^
    SetIntersection,           // &
    SetUnion,                  // + or |
}

impl fmt::Show for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::Ellipsis => write!(f, ".."),
            Op::SetDifference => write!(f, "-"),
            Op::SetSymmetricDifference => write!(f, "^"),
            Op::SetIntersection => write!(f, "&"),
            Op::SetUnion => write!(f, "+"),
        }
    }
}

enum Group {
//    Capture(Vec<char>),         // ()
    NonCapture(Vec<Ty>), // []
    CharClass(Vec<Ty>),  // <[]>
    Literal(Vec<char>),         // '' or ""
}

impl fmt::Show for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Group::NonCapture(ref vec) => write!(f, "{}", vec),
            Group::CharClass(ref vec)  => write!(f, "<{}>", vec),
            Group::Literal(ref vec)    => write!(f, "{}", vec),
        }
    }
}

// Meta characters are characters which need to be escaped
// to match themselves. For example: `\.` matches a period.
// `.` matches anything.
enum Meta {
    Char(char),             // a b c etc.
    Digit(bool),            // \d
    Dot,                    // .
    Newline(bool),          // \n
    Tab(bool),              // \t
    Whitespace(bool),       // \s
    HWhitespace(bool),      // \h
    Word(bool),             // \w
}

impl fmt::Show for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Meta::Char(ty) => write!(f, "{}", ty),
            Meta::Digit(ty) => write!(f, "digit({})", ty),
            _ => write!(f, "unimplemented"),
        }
    }
}


impl PartialEq<char> for Meta {
    fn eq(&self, c: &char) -> bool {
        match *self {
            Meta::Char(s) => s == *c,
            Meta::Digit(digit) => {
                let res = matches!(*c, '0' ... '9');
                if digit { res } else { !res }
            },
            Meta::Dot => { true },
            Meta::Tab(tab) => {
                let res = matches!(*c, '\t');
                if tab { res } else { !res }
            },
            Meta::Whitespace(white) => {
                let res = matches!(*c, ' ' | '\t' | '\n');
                if white { res } else { !res }
            },
            Meta::HWhitespace(h_white) => {
                let res = matches!(*c, ' ' | '\t');
                if h_white { res } else { !res }
            },
            Meta::Newline(is) => {
                let res = matches!(*c, '\n');
                if is { res } else { !res }
            },
            Meta::Word(word) => {
                let res = matches!(*c, 'a' ... 'z' |
                                       'A' ... 'Z' |
                                       '0' ... '9' |
                                       '_' );
                if word { res } else { !res }
            },
        }
    }
    

}

impl Meta {
    // Push characters between first 2 `'` or `"` sets into Literal.
    fn literal(open: char, chars: &mut Iterator<char>) -> Ty {
        let mut vec = vec![];
        
        while let Some(c) = chars.next() {
            match c {
                _ if c == open => return Ty::Group(Group::Literal(vec)),
                c => vec.push(c),
            }
        }

        // Characters ran out before literal was closed with ' or ".
		panic!(r"The sets of {} are unbalanced.", open);
    }

    fn char_class(chars: &mut Iterator<char>) -> Ty {
        let mut vec = vec![];
        
        while let Some(c) = chars.next() {
            match c {
                // Ignore whitespace
                ' ' | '\t' | '\n' => continue,

                // Set operators
                '-' => vec.push(Ty::Op(Op::SetDifference)),
                '^' => vec.push(Ty::Op(Op::SetSymmetricDifference)),
                '&' => vec.push(Ty::Op(Op::SetIntersection)),
                '+' => vec.push(Ty::Op(Op::SetUnion)),
                
                '[' => vec.push(Meta::non_capture(chars)),
                '>' => return Ty::Group(Group::CharClass(vec)),
                _   => panic!("`{}` is not valid inside a character class \
and outside a group.", c),
            }
        }

		panic!(r"Missing a `>` char.");
    }

    fn non_capture(chars: &mut Iterator<char>) -> Ty {
        let mut vec = vec![];
        
        while let Some(c) = chars.next() {
            match c {
                // Ignore whitespace
                ' ' | '\t' | '\n' => continue,
                
                'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_'
                            => vec.push(Ty::Meta(Meta::Char(c))),
                '.'  | '\\' => vec.push(Meta::single(c, chars)),
                ']'         => return Ty::Group(Group::NonCapture(vec)),
                c           => panic!("`{}` is an invalid in this position.", c),
            }
        }

		panic!(r"The sets of `]` are unbalanced.");
    }

    // Multiple characters which represent a single symbol.
    fn single(open: char, chars: &mut Iterator<char>) -> Ty {
        match (open, chars.next().expect("Another character is expected.")) {
            // Default character classes
            ('\\', 'd') => Ty::Meta(Meta::Digit(true)),
            ('\\', 'D') => Ty::Meta(Meta::Digit(false)),
            ('\\', 'n') => Ty::Meta(Meta::Newline(true)),
            ('\\', 'N') => Ty::Meta(Meta::Newline(false)),
            ('\\', 't') => Ty::Meta(Meta::Tab(true)),
            ('\\', 'T') => Ty::Meta(Meta::Tab(false)),
            ('\\', 's') => Ty::Meta(Meta::Whitespace(true)),
            ('\\', 'S') => Ty::Meta(Meta::Whitespace(false)),
            ('\\', 'h') => Ty::Meta(Meta::HWhitespace(true)),
            ('\\', 'H') => Ty::Meta(Meta::HWhitespace(false)),
            ('\\', 'w') => Ty::Meta(Meta::Word(true)),
            ('\\', 'W') => Ty::Meta(Meta::Word(false)),
            ('\\', c)   => Ty::Meta(Meta::Char(c)),
            
            ('.', '.')  => Ty::Op(Op::Ellipsis),
            (a, b)      => panic!(r"`{}{}` is not a valid sequence.", a, b),
        }
    }
    /*fn simplify(class: &mut Meta) -> Meta {
        let mut loc = None;
        for (i, element) in class.iter().enumerate() {
            match element {
                Meta::Union => {
                    loc = Some(i);
                    break;
                },
                _ => {},
            }
        }
        
        
        
        
    }*/
}

#[deriving(Show)]
pub struct Rule {
    regex: Vec<Ty>,
}

impl Rule {
    pub fn new(s: &str) -> Result<Rule, &'static str> {
        let mut chars = s.chars();
        let mut regex = vec![];
        
        while let Some(c) = chars.next() {
            match c {
                'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_'
                           => regex.push(Ty::Meta(Meta::Char(c))),
                '.'        => regex.push(Ty::Meta(Meta::Dot)),
                ' ' | '\t' | '\n' => continue,
                '\\'       => regex.push(Meta::single(c, &mut chars)),
                '\'' | '"' => regex.push(Meta::literal(c, &mut chars)),
                '<'        => regex.push(Meta::char_class(&mut chars)),
                '['        => regex.push(Meta::non_capture(&mut chars)),
                _          => continue,
            }
        }
        println!("regex: {}", regex[0]);
        //Meta::simplify(&mut vec[0]);
        Ok(Rule { regex: regex })
    }
    
    /*pub fn is_match(self, s: &str) -> bool {
        let mut chars = s.chars();
        let mut mode = Mode::Scan;
        
        loop {
            match mode {
                Mode::Match => {
                    let mut first = true;
                    for regex in self.regex.iter() {
                        // Scan already checked first character so skip iteration.
                        if first {
                            first = false;
                            continue;
                        }
                        
                        if let Some(c) = chars.next() {
                            println!("c: {}, regex: {}", c, regex);
                            // Issue 19035: swapping cmp order breaks code
                            if *regex != c  {
                                mode = Mode::Scan;
                                break;
                            }
                        }
                    }

                    // If finished checking without needing further scanning,
                    // then it found a match.
                    if let Mode::Match = mode { return true; }
                },
                Mode::Scan => {
                    while let Some(c) = chars.next() {
                        // Issue 19035: swapping cmp order breaks code
                        if self.regex[0] == c {
                            mode = Mode::Match;
                            break;
                        }
                    }
                    
                    // If finished scanning string but no match, there isn't a match.
                    if let Mode::Scan = mode { return false; }
                },
            };
        };
    }*/
}

#[cfg(test)]
mod test {
    use super::{CharSet, Meta};
    
    #[test]
    fn charset_alphanumerics_is_accurate() {
        
        let CharSet { chars: digits, .. }   = CharSet::new(Meta::Digit(true));
        let CharSet { chars: alphabet, .. } = CharSet::new(Meta::Word(true));
        
        let mut new_alphabet = vec![];
        let mut new_digits   = vec![];
        
        for i in range(97, 123u8) { // a-z
            new_alphabet.push(i as char);
        }
        for i in range(65, 91u8) {  // A-Z
            new_alphabet.push(i as char);
        }
        for i in range(48, 58u8) {  // 0-9
            new_alphabet.push(i as char);
            new_digits.push(i as char);
        }
        
        new_alphabet.push(95u8 as char);   // _

        // Check that typed alphabet is the same as computed
        for (a1, a2) in alphabet.chars().zip(new_alphabet.iter()) {
            assert_eq!(a1, *a2);
        }

        // Check that typed digits are the same as computed
        for (d1, d2) in digits.chars().zip(new_digits.iter()) {
            assert_eq!(d1, *d2);
        }
    }
}

