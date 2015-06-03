# range_set

A library which allows set operations for ranges.

```rust
let mut set = Set::new();
set.insert(Range('1', '3'));
set.insert(Range('2', '6')); // Ranges overlap. This has `Range('1', '6')`.
set.union(Range('8', '9'));  // `{ Range('1', '6'), Range('8', '9') }`
```
