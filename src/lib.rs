#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(slice_patterns)]

//! # Rules
//! Rules uses regular expressions to do pattern matching using syntax
//! based upon the Perl 6 regex grammar. The Perl 6 grammar has been heavily
//! revised from Perl 5 and should not be equated with it. This may look
//! nothing like any regex you have seen before.
//!
//! # Note
//!
//! The only real currently available method is [`is_match()`]
//! (re/struct.Regex.html#method.is_match).
//!
//! This is designed to operate on Unicode scalar values.
//!
//! # Syntax
//!
//! Whitespace is generally ignored so that a regex can be more readable
//! and less dense.
//!
//! ```plain
//! r"fred"    // Normal way
//! r"f r e d" // Completely equivalent
//! // Will match `apples_oranges` or any other deliminator
//! r"apples . oranges"
//! ```
//!
//! # Literals
//!
//! Alphanumerics, underscores (`_`), and everything enclosed within
//! quotes (`"`) and ticks (`'`) are the only literals.
//!
//! ```plain
//! hello_world   // Matches `hello_world`.
//! "carrot cake" // Matches `carrot cake`.
//! 'apple pie'   // Matches `apple pie`.
//! ```
//!
//! Everything else must be escaped with a backslash (`\*`) to literally match.
//!
//! ```plain
//! it\'s\ my\ birthday // Matches `it's my birthday`.
//! ```
//!
//! # Chevrons: `<>`
//!
//! Chevrons are considered a metacharacter grouping operator whose behaviour
//! changes depending on the first character found inside. The behavior for
//! each different character is:
//!
//! First character | Example | Result
//! ----------------|---------|-------
//! Whitespace | `< big small >` | Alternative quotes matches `[ 'big' | 'small' ]`
//! alphabetic | `<alpha>` | Named character class which capture
//! `?` | `<?before foo>` | A positive zero width assertion
//! `!` | `<!before foo>` | A negative zero width assertion
//! `[` | `<[ ab ]>` | A character class matches `[ 'a' | 'b' ]`
//! `-` | `<-[a] + [b]>` | Negated character class: `[ab]` negated
//! `+` | `<+ [a] >` | Doesn't modify the class.
//!
//! # Lookaround
//!
//! * lookahead - `foo <?after bar>` matches `foo` in `foobar`
//! * negative lookahead - `foo <!after bar>` matches `foo` in `foobaz`
//! * lookbehind - `<?before foo> bar` matches `bar` in `foobar`
//! * negative lookbehind - `<!before foo> bar` matches `bar` in `sushibar`
//!
//! An example with both:
//! `<?before foo> bar <?after baz>`  matches `bar` in `foobarbaz`
//!
//! # Set operators
//!
//! These operators can be applied to groups which will be analyzed later:
//!
//! ```plain
//! +       Union                // [123] + [345] = [12345]
//! |       Union                // Same
//! &       Intersection         // [123] & [345] = [3]
//! -       Difference           // [123] - [345] = [12]
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
//! ```
//!
//! # Comments
//!
//! Comments are allowed inside a regex.
//!
//! ```plain
//! // This matches `myregex`
//! r"my # This is a comment which goes to the end of the line
//! regex"
//! ```

#[doc(hidden)] pub mod collapse;
#[doc(hidden)] pub mod parse;
#[doc(hidden)] pub mod range_set;
#[doc(hidden)] pub mod unicode;

pub mod re;
