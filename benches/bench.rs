use bencher::{benchmark_group, benchmark_main, Bencher};
use lazy_static::lazy_static;

lazy_static! {
    static ref STRS: Vec<&'static str> = vec!["\tRust\t"; 10_000];
}

// Use `collect` and then `join`
fn a(bench: &mut Bencher) {
    bench.iter(|| -> String {
        (*STRS)
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(", ")
    })
}

// Use `itertools::Itertools::intersperse`
fn b(bench: &mut Bencher) {
    use itertools::Itertools;
    bench.iter(|| -> String { (*STRS).iter().copied().intersperse(", ").collect() })
}

// Use `fold` (WITHOUT specifying String size)
fn c(bench: &mut Bencher) {
    bench.iter(|| -> String {
        (*STRS)
            .iter()
            .map(|s| s.trim())
            .fold(String::new(), |mut acc, cur| {
                acc.push_str(cur);
                acc.push_str(", ");
                acc
            })
    })
}

// Use `fold` (WITH specifying String size)
fn d(bench: &mut Bencher) {
    bench.iter(|| -> String {
        (*STRS).iter().map(|s| s.trim()).fold(
            String::with_capacity((*STRS).get(0).unwrap().len() * (*STRS).len()),
            |mut acc, cur| {
                acc.push_str(cur);
                acc.push_str(", ");
                acc
            },
        )
    })
}

// Use `fold` & `+` operator (WITH specifying String size)
fn e(bench: &mut Bencher) {
    bench.iter(|| -> String {
        (*STRS).iter().map(|s| s.trim()).fold(
            String::with_capacity((*STRS).get(0).unwrap().len() * (*STRS).len()),
            |acc, cur| acc + cur + ", ",
        )
    })
}

// Use `itertools::join`
fn f(bench: &mut Bencher) {
    use itertools::Itertools;
    bench.iter(|| -> String { (*STRS).iter().map(|s| s.trim()).join(", ") })
}

benchmark_group!(benches, a, b, c, d, e, f);
benchmark_main!(benches);
