use day_4::*;
use std::fs;

#[test]
fn test_input() {

    let table = parse_input(
        fs::read_to_string("input.txt").expect("Failed to access file.")
    );

    let result = window(table);

    assert_eq!(result, 1508);
}
