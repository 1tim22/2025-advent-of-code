use day_6::*;

#[test]
fn test_example_input() {
    let input = "123 328  51 64
                  45 64  387 23
                   6 98  215 314
                 *   +   *   +";

    let numbers   = parse_input(input.to_string());
    let operators = parse_input_operators(input.to_string());

    println!("numbers:   {:?}", numbers);
    println!("operators: {:?}", operators);

    let result = arithmetic(numbers, operators);

    assert_eq!(result, 4277556);
}
