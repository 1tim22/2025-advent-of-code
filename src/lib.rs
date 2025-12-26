use regex::Regex;
use std::ops::RangeInclusive;

pub fn count_fresh(ranges: Vec<RangeInclusive::<u64>>, ingredients: Vec<u64>) -> u64 {
    let fresh_ingredients = ingredients.iter().map(|id|
        ranges.iter().map(|range| match range.contains(id) {
            true => 1,
            _    => 0
        }).sum::<u64>()
    ).collect::<Vec<u64>>();

    // This is a bit unnecessary, but I am trying to anticipate part 2
    fresh_ingredients.iter().map(|&count| if count > 0 { 1 } else { 0 })
        .sum()
}

pub fn count_fresh_possible(ranges: Vec<RangeInclusive::<u64>>) -> usize {
    let mut test = ranges.iter().flat_map(|range| {
        range.clone().collect::<Vec<u64>>()
    }).collect::<Vec<u64>>();

    test.sort();
    test.dedup();

    println!("{:?}", test);

    test.len()
}

pub fn parse_input_ranges(contents: String) -> Vec<RangeInclusive::<u64>> {
    Regex::new(r"(\d+)[-](\d+)")
        .expect("Invalid RegEx")
        .captures_iter(contents.as_str())
        .map(|matches| {
            let (_, [start, end]) = matches.extract();

            start.parse::<u64>().expect("Could not parse beginning of range.")
            ..=
            end.parse::<u64>().expect("Could not parse end of range.")
        }).collect::<Vec<RangeInclusive::<u64>>>()
}

pub fn parse_input_ingredients(contents: String) -> Vec<u64> {
    contents.split_whitespace()
        .filter(|row| !row.contains('-'))
        .map(|id| id.parse::<u64>().expect("Could not parse ingredient ID."))
        .collect::<Vec<u64>>()
}
