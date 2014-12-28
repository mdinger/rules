#![feature(macro_rules)]
#![feature(slicing_syntax)]

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

pub struct Rule {
    regex: String,
}

enum Mode {
    Match,
    Scan,
}

impl Rule {
    pub fn new(s: &str) -> Result<Rule, ()> {
        Ok(Rule { regex: s.to_string() })
    }
    
    pub fn is_match(self, s: &str) -> bool {
        let mut chars = s.chars();
        let mut mode = Mode::Scan;
        
        loop {
            match mode {
                Mode::Match => {
                    let mut count = 0u;
                    while let Some(c) = chars.next() {
                        count += 1;
                        if count == self.regex.len() { break; }
                        else if c != self.regex[].char_at(count) {
                            mode = Mode::Scan;
                            break;
                        }
                    }

                    // If finished checking without needing further scanning,
                    // then it found a match.
                    if let Mode::Match = mode { return true; }
                },
                Mode::Scan => {
                    while let Some(c) = chars.next() {
                        if c == self.regex[].char_at(0) {
                            mode = Mode::Match;
                            break;
                        }
                    }
                    
                    // If finished scanning string but still needs scanning,
                    // then there isn't a match in the str.
                    if let Mode::Scan = mode { return false; }
                },
            };
        };
    }
}

