#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(default_type_params)]

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

#[deriving(Show)]
pub struct Rule {
    regex: Vec<CharClass>,
}

enum Mode {
    Match,
    Scan,
}

#[deriving(Show)]
enum CharClass {
    Char(char),
    Digit(bool),
    Dot,
    Newline(bool),
    Tab(bool),
    Whitespace(bool),
    HWhitespace(bool),
    Word(bool),
}

impl PartialEq<char> for CharClass {
    fn eq(&self, c: &char) -> bool {
        match *self {
            CharClass::Char(s) => s == *c,
            CharClass::Digit(digit) => {
                let res = matches!(*c, '0' ... '9');
                if digit { res } else { !res }
            },
            CharClass::Dot => { true },
            CharClass::Tab(tab) => {
                let res = matches!(*c, '\t');
                if tab { res } else { !res }
            },
            CharClass::Whitespace(white) => {
                let res = matches!(*c, ' ' | '\t' | '\n');
                if white { res } else { !res }
            },
            CharClass::HWhitespace(h_white) => {
                let res = matches!(*c, ' ' | '\t');
                if h_white { res } else { !res }
            },
            CharClass::Newline(is) => {
                let res = matches!(*c, '\n');
                if is { res } else { !res }
            },
            CharClass::Word(word) => {
                let res = matches!(*c, 'a' ... 'z' |
                                       'A' ... 'Z' |
                                       '0' ... '9' |
                                       '_' );
                if word { res } else { !res }
            },
        }
    }
}

impl Rule {
    pub fn new(s: &str) -> Result<Rule, &'static str> {
        let mut chars = s.chars();
        let mut regex: Vec<CharClass> = vec![];
        
        while let Some(c) = chars.next() {
            match c {
                'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_'
                     => regex.push(CharClass::Char(c)),
                '.'  => regex.push(CharClass::Dot),
                ' ' | '\t' | '\n' => { continue; },
                '\\' => {
                    let char_class = match chars.next() {
                        Some('d') => CharClass::Digit(true),
                        Some('D') => CharClass::Digit(false),
                        Some('h') => CharClass::HWhitespace(true),
                        Some('H') => CharClass::HWhitespace(false),
                        Some('n') => CharClass::Newline(true),
                        Some('N') => CharClass::Newline(false),
                        Some('s') => CharClass::Whitespace(true),
                        Some('S') => CharClass::Whitespace(false),
                        Some('t') => CharClass::Tab(true),
                        Some('T') => CharClass::Tab(false),
                        Some('w') => CharClass::Word(true),
                        Some('W') => CharClass::Word(false),
                        Some(c)   => CharClass::Char(c),
                        None => { return Err(r"Last character (`\`) is invalid") },
                    };
                    
                    regex.push(char_class);
                },
                '\'' => {
                	let mut closed = false;
                    while let Some(c) = chars.next() {
                        match c {
                            '\'' => {
                            	closed = true;
                            	break;
                        	}
                            c => regex.push(CharClass::Char(c)),
                        }
                    }

					if !closed { return Err(r"The sets of `'` are unbalanced.") }
                },
                '"' => {
                	let mut closed = false;
                    while let Some(c) = chars.next() {
                        match c {
                            '"' => {
                            	closed = true;
                            	break;
                        	}
                            c => regex.push(CharClass::Char(c)),
                        }
                    }

                    if !closed { return Err(r#"The sets of `"` are unbalanced."#) }
                },
                _ => { continue; },
            }
        }
        
        Ok(Rule { regex: regex })
    }
    
    pub fn is_match(self, s: &str) -> bool {
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
    }
}

