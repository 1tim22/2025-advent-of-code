use day_3::*;
use std::fs;

#[test]
fn test_input() {

    let table = parse_input(
        fs::read_to_string("input.txt").expect("Failed to access file.")
    );

    println!("{:?}", table);

    let result = calculate_joltage(table);

    println!("{:?}", result);

    assert_eq!(result, 17412);
}
