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
test a ... bench:     106,045 ns/iter (+/- 16,800)
test b ... bench:     162,460 ns/iter (+/- 23,986)
test c ... bench:     174,002 ns/iter (+/- 22,692)
test d ... bench:     167,454 ns/iter (+/- 15,830)
test e ... bench:     171,400 ns/iter (+/- 25,176)
test f ... bench:     327,217 ns/iter (+/- 36,332)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured
```

### Summary

| Test | About                                               | Performance (ns) lower is better | Error      |
| ---- | --------------------------------------------------- | -------------------------------- | ---------- |
| A    | `collect` and then `join`                           | 106,045                          | +/- 16,800 |
| B    | `itertools::Itertools::intersperse`                 | 162,460                          | +/- 23,986 |
| C    | `fold` (WITHOUT specifying String size)             | 174,002                          | +/- 22,692 |
| D    | `fold` (WITH specifying String size)                | 167,454                          | +/- 15,830 |
| E    | `fold` & `+` operator (WITH specifying String size) | 171,400                          | +/- 25,176 |
| F    | `itertools::join`                                   | 327,217                          | +/- 36,332 |

<img width="829" alt="benchmark" src="https://user-images.githubusercontent.com/23649474/71777106-edf93980-2fde-11ea-909f-8fbc5ab3c25b.png">

## Conclusion

- Using `itertools:join` your code will be the simplest and most readable, but it is much the slowest.
- When high performance is required, a basic style like `<YOUR_ITERATOR>.collect::<Vec<_>>().join(", ")` is recommended.
