use day_3::*;

#[test]
fn test_example_input() {

    let table = parse_input("
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ".to_string());

    println!("{:?}", table);

    let result = calculate_joltage(table);

    println!("{:?}", result);

    assert_eq!(result, 357);
}
