use regex::Regex;

pub fn calculate_dial(input: Vec<i32>, start: i32, size: i32) -> i32 {
    let mut zeros = 0;

    println!("x	sum	result");
    input.iter().fold(start, |acc, &x| {
        let mut result = (1 + size + (acc + x)) % (1 + size);

        // This should be resolved by the above equation itself
        if result < 0 {
            result += size + 1;
        };

        println!("{:?}	{:?}	{:?}", x, (acc + x), result);

        // Count the number of zeros encountered
        if 0 == result {
            zeros += 1;
        }

        result
    });

    zeros
}

pub fn parse_numbers(contents: String) -> Vec<i32> {
    Regex::new(r"(\w)(\d+)")
        .expect("Invalid RegEx")
        .captures_iter(contents.as_str())
        .map(|matches| {
            let (_, [direction, number]) = matches.extract();

            (match direction {
                "L" => -1, // Left is a negative direction
                _   =>  1  // Right is a positive direction
            }) * number.parse::<i32>().expect("Could not parse number.")
        }).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_example_input() {
        let total = calculate_dial(vec![
            -68,
            -30,
            48,
            -5,
            60,
            -55,
            -1,
            -99,
            14,
            -82,
        ], 50, 99);

        assert_eq!(total, 3);
    }

    #[test]
    fn test_example_modified_input() {
        let total = calculate_dial(vec![
            -68,
            -30,
            48,
            -5,
            60,
            -55,
            -1,
            -99,
            14,
            -114,
            -115,
            -82,
        ], 50, 99);

        assert_eq!(total, 4);
    }

    #[test]
    fn test_puzzle_input() {
        let total = calculate_dial(parse_numbers(fs::read_to_string("input.txt").expect("Failed to access file.")), 50, 99);

        assert_eq!(total, 1150);
    }
}
