# rules

A Rust implementation of Perl 6 regex rules

[![Build Status](https://travis-ci.org/mdinger/rules.svg?branch=master)](https://travis-ci.org/mdinger/rules)

[Documentation](http://mdinger.github.io/rules/)

Note: if you just want a good implementation of a regex, use
[regex](https://crates.io/crates/regex). This is an experimental library which
currently would likely be entirely inadequate for your purposes. Hopefully it
will be a good library someday.

This is an implementation of
[Perl 6 rules](http://en.wikipedia.org/wiki/Perl_6_rules) for Rust. Perl 6
was supposed to break anything while ignoring backwards compatibility. Thus, the
*regex* from Perl 5 was improved into what they are now calling a framework of
*rules*. This provides more flexibility than before and is syntactically different.

Speed is not currently a priority. If it's fast, great. If it isn't, so be it. I'm
mainly trying to work towards getting the syntax working properly. If the syntax
works properly, it can be experimented with even if it isn't used seriously. This
is important because while regular regex are kinda powerful, they are really
difficult to read, may do many non-obvious things, and things like literal whitespacing
makes large regex really difficult to work with. rules changes a lot of those aspects
and so hopefully makes them much more usable.

Note: this is first and foremost a *Rust* library and as such it is valid to deviate from
the Perl 6 spec given a proper reason, especially if it makes it a better fit to the Rust
system. However, I will try to keep such deviations to a minimum since Perl 6 is probably
very well thought out and minor inconsistencies between languages only makes things more
confusing when you must switch between languages (this is especially problematic with
regex where every different version uses subtly different syntax and has different features).
Deviations from the spec will be specified in `perl_changes.md`.

The cargo built docs are not necessarily current with the library. I scope out the
official Perl6 docs for how syntax works and update my *local* docs to match what I
understand. Then I base the library on the *local* docs. So the local docs should
always run ahead of what the library can do. For examples of what the library can
do, look at the tests in `tests/re`.

The main perl 6 regex docs are at
[doc.perl6.org](http://doc.perl6.org/language/regexes).
The regex design docs are at [S05](http://design.perl6.org/S05.html) which is really
helpful when trying to interpret parts of the spec.

A another different intro to Perl 6 rules is found
[here](https://github.com/perlpilot/perl6-docs/blob/master/intro/p6-regex-intro.pod).

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
rules = "*"
```

and this to your crate root:

```rust
extern crate rules;
```

## License
Dual licensed under MIT and Apache. See LICENSE-APACHE and LICENSE-MIT for details.
