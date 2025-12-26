use day_5::*;
use std::fs;

#[test]
fn test_input() {
    let input = fs::read_to_string("input.txt").expect("Failed to access file.");

    let ranges = parse_input_ranges(input.to_string());

    let result = count_fresh_possible(ranges);

    // assert_eq!(result, 307132225096046); // Too low
    assert_eq!(result, 344771884978261);
    // assert_eq!(result, 654602082915594); // Too high
}
