#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(slice_patterns)]

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
//! <[0-6] ^ [5-9]>   // Matches a `5` or `6`
//! <-[0-6]>          // Negation: anything but 0-6
//! ```
//!
//! # Comments
//!
//! Comments are allowed inside a regex.
//!
//! ```plain
//! // This matches `myregex`
//! r"my // This is a comment which goes to the end of the line
//! regex"
//! ```

pub mod collapse;
pub mod parse;
pub mod re;
pub mod range_set;
pub mod unicode;

