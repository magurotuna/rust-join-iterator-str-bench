fn main() {
    println!("Hello, world!");
}

#[test]
fn test_result_equivalent() {
    use itertools::Itertools;

    let strs = vec!["\tRust\t"; 5];

    let expected = "Rust, Rust, Rust, Rust, Rust";
    let expected_with_trail = "Rust, Rust, Rust, Rust, Rust, ";

    let a_result = strs.iter().map(|s| s.trim()).collect::<Vec<_>>().join(", ");
    let b_result: String = strs.iter().map(|s| s.trim()).intersperse(", ").collect();
    let c_result = strs
        .iter()
        .map(|s| s.trim())
        .fold(String::new(), |mut acc, cur| {
            acc.push_str(cur);
            acc.push_str(", ");
            acc
        });
    let d_result = strs.iter().map(|s| s.trim()).fold(
        String::with_capacity(strs.get(0).unwrap().len() * strs.len()),
        |mut acc, cur| {
            acc.push_str(cur);
            acc.push_str(", ");
            acc
        },
    );
    let e_result = strs.iter().map(|s| s.trim()).fold(
        String::with_capacity(strs.get(0).unwrap().len() * strs.len()),
        |acc, cur| acc + cur + ", ",
    );
    let f_result = strs.iter().map(|s| s.trim()).join(", ");

    assert_eq!(expected, a_result);
    assert_eq!(expected, b_result);
    assert_eq!(expected_with_trail, c_result);
    assert_eq!(expected_with_trail, d_result);
    assert_eq!(expected_with_trail, e_result);
    assert_eq!(expected, f_result);
}
