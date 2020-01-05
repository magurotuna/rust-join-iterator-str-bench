# rust-join-iterator-str-bench

In Rust, We often want to join string iterators like this:

INPUT:

```rust
["foo", "bar", "baz"].iter().map(|s|
    s.trim() // do various things here
)
```

OUTPUT:

```rust
"foo, bar, baz"
```

There are several ways to realize this. But which way is the fastest? So I make a benchmark survey.

## Benchmark result

### My envirionment

- OS: macOS Catalina v10.15.1
- CPU: Core i7-8569U
- Memory: 16GB
- Rust: cargo & rustc v1.40

### Output whan running `cargo bench`

```
running 6 tests
test a ... bench:     110,924 ns/iter (+/- 10,779)
test b ... bench:     164,369 ns/iter (+/- 19,445)
test c ... bench:     180,906 ns/iter (+/- 23,064)
test d ... bench:     178,038 ns/iter (+/- 16,209)
test e ... bench:     177,628 ns/iter (+/- 17,991)
test f ... bench:     379,071 ns/iter (+/- 82,040)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured
```

### Summary

| Test | About                                               | Performance (ns) lower is better | Error      |
| ---- | --------------------------------------------------- | -------------------------------- | ---------- |
| A    | `collect` and then `join`                           | 110,924                          | +/- 10,779 |
| B    | `itertools::Itertools::intersperse`                 | 164,369                          | +/- 19,445 |
| C    | `fold` (WITHOUT specifying String size)             | 180,906                          | +/- 23,064 |
| D    | `fold` (WITH specifying String size)                | 178,038                          | +/- 16,209 |
| E    | `fold` & `+` operator (WITH specifying String size) | 177,628                          | +/- 17,991 |
| F    | `itertools::join`                                   | 379,071                          | +/- 82,040 |

<img width="702" alt="benchmark summary" src="https://user-images.githubusercontent.com/23649474/71775538-c8ac0180-2fc5-11ea-9ef2-45d33224d546.png">

## Conclusion

- Using `itertools:join` your code will be the simplest and most readable, but it is much the slowest.
- When high performance is required, a basic style like `<YOUR_ITERATOR>.collect::<Vec<_>>().join(", ")` is recommended.
