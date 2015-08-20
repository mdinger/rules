use parse::{self, Ast};
use collapse;

#[derive(Debug)]
pub struct Regex(Vec<Ast>);

impl Regex {
    pub fn new(s: &str) -> Regex {
        let vec = parse::parse(s).unwrap();
        let vec = collapse::collapse(vec);

        Regex(vec)
    }
    /// Returns true if and only if the regex matches the string given.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate rules; use rules::re::Regex;
    /// # fn main() {
    /// let pineapple = Regex::new(r"pine
    ///                              apple");
    /// assert!(pineapple.is_match("I bought a pineapple."));
    ///
    /// // Digit or `a`. `-` is set difference.
    /// let char_class = Regex::new(r"<[ 0 .. 9 a .. z] - [ b .. z ]>");
    /// assert!(char_class.is_match("3"));
    /// assert!(char_class.is_match("a"));
    ///
    /// assert!(!char_class.is_match("b"));
    /// # }
    /// ```
    pub fn is_match(&self, s: &str) -> bool {
        let match_start = MatchStart { ast: &self.0[0], chars: s, cur: 0 };
        let mut matches = false;

        // Iterates over the substrings which could *possibly* match based
        // only on the first Ast.
        for substr in match_start {
            let mut substr = substr;

            for ast in &self.0 {
                // No way to return a result based on if the loop doesn't break. This
                // is the workaround.
                matches = true;

                if let Some(trimmed) = ast.trim_left_match(substr) {
                    substr = trimmed;
                } else {
                    matches = false;
                    break
                }
            }

            if matches { return true }
        }

        false
    }
}

// A struct to allow an Iterator to be created which will return substrings
// which start where a single Ast matches.
struct MatchStart<'a> {
    ast: &'a Ast,
    chars: &'a str,
    cur:  usize,
}

impl<'a> Iterator for MatchStart<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if let Some(pos) = self.ast.find(&self.chars[self.cur..]) {
            self.cur += pos + 1;

            Some(&self.chars[self.cur-1..])
        } else { None }
    }
}
