use day_5::*;

#[test]
fn test_example_input() {
    let input = "
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    ";

    let ranges      = parse_input_ranges(     input.to_string());
    let ingredients = parse_input_ingredients(input.to_string());

    // println!("{:?}", ranges);
    // println!("{:?}", ingredients);

    let result = count_fresh(ranges, ingredients);

    assert_eq!(result, 3);
}
