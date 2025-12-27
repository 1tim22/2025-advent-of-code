use day_6::*;
use std::fs;

#[test]
fn test_input() {
    let input = fs::read_to_string("input.txt").expect("Failed to access file.");

    let numbers   = parse_input(input.to_string());
    let operators = parse_input_operators(input.to_string());

    println!("numbers:   {:?}", numbers);
    println!("operators: {:?}", operators);

    let result = arithmetic(numbers, operators);

    assert_eq!(result, 7996218225744);
}
