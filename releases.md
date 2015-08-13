## 0.0.2

A bunch of documentation changes to make it slightly usable for
new people and got travis working properly.

* travis now
  * runs properly at commit push
  * builds docs at commit push
* the readme now
  * has a usage example
  * has a link to the docs
  * has a travis icon showing build status
* the docs now
  * only show public functionality
  * has an example for `is_match()`

## 0.0.1 - Initial public release

* code available on github
* the type `Regex` has 2 working functions:
  * `is_match()`
  * `new()`
* A regex parse and match is valid for:
  * ignoring whitespace
  * handling character escapes such as
    * `\d` and `\D`
    * `\h` and `\H`
    * `\n` and `\N`
    * `\s` and `\S`
    * `\t` and `\T`
    * `\w` and `\W`
  * parsing literals via:
    * `'`: ticks
    * `"`: quotes
  * parsing character classes (`<[ ]>`)
    * with character ranges via `..`
  * using set ops on character class subsets such as
    * union via `+` or `|`
    * intersection via `&`
    * difference via `-`
    * symmetric difference via `^`
