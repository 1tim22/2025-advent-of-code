use day_2::*;
use std::fs;

#[test]
fn test_input() {
    let ranges = parse_ranges(
        fs::read_to_string("input.txt").expect("Failed to access file.")
    );

    let table = expand_ranges(ranges);

    let count = count_invalid(&table);

    assert_eq!(count, 32976912643);
}
