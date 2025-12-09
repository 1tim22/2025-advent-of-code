// The instructions state "You cannot rearrange batteries"
// which I take to mean that I am not allowed to sort iterables.

// The basic algorithm is to find the greatest value in a slice of the bank
// (bank length - 1) ignoring the last value
// to then split the bank at the point of the identified value and find the
// next largest value in the second slice (to the right of the initial value).
// The final step is to simply combine these two numbers into a single number
// and sum all of them together
pub fn calculate_joltage(banks: Vec<Vec<u32>>) -> u32 {
    let bank_iter = banks.into_iter().map(|bank| {
        let pair: (usize, &u32) = bank.split_last().expect("Is the battery bank empty?")
            .1.iter().enumerate()
            .reduce(|pair, (index, x)| if x > pair.1 { (index, x) } else { pair })
            .unwrap_or((0, &0));

        println!("Split {:?} at {:?} -> {:?}", pair.1, pair.0, bank.split_at(pair.0 + 1).1);

        let x1: &u32 = bank.split_at(pair.0 + 1).1.iter().max().unwrap_or(&0);

        // Calculate joltage by joining both the first and second numbers
        (10 * pair.1) + x1
    });

    println!("{:?}", bank_iter);

    // Sum all of the joltages for all of the battery banks
    bank_iter.sum()
}

pub fn parse_input(contents: String) -> Vec<Vec<u32>> {
    contents.split_whitespace()
        .map(|row| row.chars().map(|value|
            value.to_digit(10).expect("Could not convert to base-10 digit.")
        ).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}
