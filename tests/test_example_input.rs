use day_3::*;

#[test]
fn test_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    let result = calculate_joltage(table, 12);

    assert_eq!(result, 3121910778619);
}

#[test]
fn test_recursion_size_1_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    let result = calculate_joltage(table, 1);

    assert_eq!(result, 35);
}

#[test]
fn test_recursion_size_2_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    let result = calculate_joltage(table, 2);

    assert_eq!(result, 357);
}

#[test]
fn test_recursion_size_3_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    let result = calculate_joltage(table, 3);

    assert_eq!(result, 3205);
}

#[test]
fn test_recursion_size_4_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    let result = calculate_joltage(table, 4);

    assert_eq!(result, 31684);
}

#[test]
fn test_recursion_size_5_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    let result = calculate_joltage(table, 5);

    assert_eq!(result, 316473);
}
