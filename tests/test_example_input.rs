use day_2::*;
use fancy_regex::Regex;

#[test]
fn test_parse_ranges() {
    let ranges = parse_ranges(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
    );

    println!("length: {:?}, {:?}", ranges.len(), ranges);

    assert_eq!(ranges.len(), 11);
}

#[test]
fn test_example_input() {
    let ranges = parse_ranges(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
    );

    let table = expand_ranges(ranges);

    let count = count_invalid(&table);

    println!("Invalid Count: {:?}", count);

    assert_eq!(count, 4174379265);
}

#[test]
fn test_range() {
    let _ = (1..=10).for_each(|value| println!("{:?}, ", value));
}

#[test]
fn test_regex() {
    let test = Regex::new(r"(\d+)\1")
        .expect("Invalid RegEx")
        // .captures_iter("ababcdababcd")
        .captures_iter("12341234")
        .map(|captures| {
            captures.expect("No captures")
                .iter()
                .map(|capture| capture.expect("More cowbell").as_str())
                .collect::<Vec<&str>>()
        }).collect::<Vec<Vec<&str>>>();

    println!("{:?}", test);
}
