use day_3::*;
use std::fs;

#[test]
fn test_input() {

    let table = parse_input(
        fs::read_to_string("input.txt").expect("Failed to access file.")
    );

    let result = calculate_joltage(table, 12);

    assert_eq!(result, 172681562473501);
}
