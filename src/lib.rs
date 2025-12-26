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

pub fn count_fresh_possible(mut ranges: Vec<RangeInclusive::<u64>>) -> usize {
    // This might not be necessary
    ranges.sort_by(|r0, r1| r0.start().cmp(r1.start()));

    // Search for any overlapping start of ranges
    let ranges_unique = ranges.iter().fold(vec![], |mut list: Vec<RangeInclusive::<u64>>, range| {
        let index_overlap = list.iter().position(|next|
            next.contains(range.start()) || next.contains(range.end())
        );

        if None == index_overlap {
            // If no overlapping ranges are found, add this range to the list
            list.push(range.clone());
        } else {
            // If an overlapping range was found,
            let index   = index_overlap.expect("Could not unwrap overlap index.");
            let overlap = list.get(index).expect("Could not unwrap overlap range.");

            // create a new range which includes both overlapping ranges
            let start = if overlap.contains(range.start()) {
                *overlap.start()
            } else {
                *range.start()
            };

            let end = if overlap.contains(range.end()) {
                *overlap.end()
            } else {
                *range.end()
            };

            // Replace overlapping range with new range which includes both ranges
            list.remove(index);
            list.push(start..=end);
        }

        list
    });

    ranges_unique.iter().map(|range| range.clone().count()).sum()
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
