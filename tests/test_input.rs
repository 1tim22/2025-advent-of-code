use day_5::*;
use std::fs;

#[test]
fn test_input() {
    let input = fs::read_to_string("input.txt").expect("Failed to access file.");

    let ranges      = parse_input_ranges(     input.to_string());
    let ingredients = parse_input_ingredients(input.to_string());

    let result = count_fresh(ranges, ingredients);

    assert_eq!(result, 821);
}
