use fancy_regex::Regex;

pub fn count_invalid(table: &Vec<Vec<i64>>) -> i64 {
    let invalids: Vec<&i64> = table.iter().map(|range|
        // Filter ranges that contain invalid numbers
        range.iter().filter(|value|
            Regex::new(r"^(\d+)\1$")
                .expect("Invalid RegEx")
                .is_match(&value.to_string())
                .expect("Danger Will Robinson!")
        ).collect::<Vec<&i64>>()
    ).flatten()
    .collect();

    println!("Table:   {:?}", table);
    println!("Invalid: {:?}", invalids);

    invalids.iter().map(|&value| value).sum()
}

pub fn expand_ranges(ranges: Vec<(i64, i64)>) -> Vec<Vec<i64>> {
    ranges.iter()
        .map(|&(start, end)| start..=end)
        .map(|range| range.collect::<Vec<i64>>())
        .collect()
}

pub fn parse_ranges(contents: String) -> Vec<(i64, i64)> {
    Regex::new(r"(\d+)\s*-\s*(\d+)")
        .expect("Invalid RegEx")
        .captures_iter(contents.as_str())
        .map(|captures| {
            let groups = captures.expect("No captures");
            let start  = groups.get(1).expect("No group 1").as_str();
            let end    = groups.get(2).expect("No group 2").as_str();

            println!("Start: {:?}, End: {:?}", start, end);

            (
                start.parse::<i64>().expect("Could not parse start of range."),
                end.parse::<i64>().expect("Could not parse end of range.")
            )
        }).collect::<Vec<(i64, i64)>>()
}
