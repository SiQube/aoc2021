static TEST_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test.txt"));
static INPUT_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"));

fn compute_part1(filename: &str) -> i32 {
    let (hor, dpt) =
        filename
            .lines()
            .map(|l| l.split_once(" ").unwrap())
            .fold((0, 0), |(h, d), (k, v)| {
                match (k, v.parse::<i32>().unwrap()) {
                    ("forward", v) => (h + v, d),
                    ("down", v) => (h, d + v),
                    ("up", v) => (h, d - v),
                    _ => unreachable!(),
                }
            });
    return hor * dpt;
}

fn compute_part2(_filename: &str) -> i32 {
    return 0;
}

pub fn main() {
    let test_expected_part1: i32 = 150;
    let test_result_part1 = compute_part1(TEST_STR);
    println!("+++ part 1");
    if test_result_part1 == test_expected_part1 {
        println!("{}", compute_part1(INPUT_STR));
    } else {
        println!("expected {} got {}", test_expected_part1, test_result_part1);
    };
    println!("+++ part 2");
    let test_expected_part2: i32 = 0;
    let test_result_part2 = compute_part2(TEST_STR);
    if test_result_part2 == test_expected_part2 {
        println!("{}", compute_part2(INPUT_STR));
    } else {
        println!("expected {} got {}", test_expected_part2, test_result_part2);
    };
}
