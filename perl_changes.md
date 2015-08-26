# Differences from Perl 6 by design

## Lookaround

> This is subject to reevaluation after experience has been gained using this.
My expectation is that the Perl 6 method makes perfect sense to the implementor
correlating well with names such as *lookbehind* and such. The user will have a
different perspective and I am not convinced they will like inverting the
typical *before*/*after* pattern as specified.

In perl6, lookaround assertions have `before` and `after` on the
wrong side of the leading to weird things like them both being
backwards when both are used. For example in Perl 6:

```perl
# lookahead
say "foobar" ~~ / foo <?before bar> /; # foo
# lookbehind
say "foobar" ~~ / <?after foo> bar /; # bar
# both
say "foobarbaz" ~~ / <?after foo> bar <?before baz> /; # bar
```

I find this enormously confusing and it throws me every time because
when I see `before`, I *always* interpret it backwards and I have to
manually invert it to comprehend it. I'd invert it like so:

```perl
// lookahead
say "foobar" ~~ / foo <?after bar> /; # foo
// lookbehind
say "foobar" ~~ / <?before foo> bar /; # bar
// both
say "foobarbaz" ~~ / <?before foo> bar <?after baz> /; # bar
```
