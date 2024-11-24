#![feature(array_windows)]

static TEST_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test.txt"));
static INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

fn compute_part1(filename: &str) -> i32 {
    let result: i32 = filename
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count()
        .try_into()
        .unwrap();
    return result;
}

fn compute_part2(filename: &str) -> i32 {
    let result: i32 = filename
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>()
        .array_windows()
        .filter(|[a, _, _, b]| a < b)
        .count()
        .try_into()
        .unwrap();
    return result;
}

pub fn main() {
    let test_expected_part1: i32 = 7;
    let test_result_part1 = compute_part1(TEST_STR);
    println!("+++ part 1");
    if test_result_part1 == test_expected_part1 {
        println!("{}", compute_part1(INPUT_STR));
    } else {
        println!("expected {} got {}", test_expected_part1, test_result_part1);
    };
    println!("+++ part 2");
    let test_expected_part2: i32 = 5;
    let test_result_part2 = compute_part2(TEST_STR);
    if test_result_part2 == test_expected_part2 {
        println!("{}", compute_part2(INPUT_STR));
    } else {
        println!("expected {} got {}", test_expected_part2, test_result_part2);
    };
}
