use regex::Regex;

pub fn calculate_dial(input: Vec<i32>, start: i32, size: i32) -> i32 {
    // Because the dial includes position "0" the length to be used by the algorithm is size + 1
    let length = 1 + size;

    let (_end_dial_position, zeros) = input.iter().fold((start, 0), |(position, zeros), &delta| {
        let mut result = (length + (position + delta)) % length;

        // This should be resolved by the above equation itself
        // Because the safe dial only has numbers 0 - 99 (or whatever size is),
        // it does not make sense for there to be negative positions
        if result < 0 {
            result += length;
        };

        (result, zeros + count_zeros(length, position, delta))
    });

    zeros
}

pub fn count_zeros(length: i32, position: i32, delta: i32) -> i32 {
    let distance_to_zero = if 0 > delta {
        position
    } else {
        length - position
    };

    // Calculate the offset necessary to move the dial position to "0"
    let offset = distance_to_zero - delta.abs();

    // Calculate the number of turns past zero
    // This is the clever bit—once the dial is offset to "0" it does not matter which direction is
    // traversed to calculate the number of revolutions the dial makes. Thus, the absolute value
    // may be used to calculate the number of turns past zero for a given change in the dial.
    let turns = offset.abs() / length;

    let zeros = if 0 != distance_to_zero && delta.abs() >= distance_to_zero {
        1 + turns
    } else {
        turns
    };

    println!("- position: {:?},	|delta|: {:?}, 	distance_to_zero: {:?},	offset: {:?},	turns: {:?},	{:?}", position, delta.abs(), distance_to_zero, offset, turns, zeros);

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
        let total = calculate_dial(
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82],
            50,
            99
        );

        assert_eq!(total, 6);
    }

    #[test]
    fn test_example_modified_input() {
        let total = calculate_dial(
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -114, -115, -82, 99, -999],
            50,
            99
        );

        assert_eq!(total, 19);
    }

    #[test]
    fn test_puzzle_input() {
        let total = calculate_dial(parse_numbers(fs::read_to_string("input.txt").expect("Failed to access file.")), 50, 99);

        // assert_eq!(total, 6759);
        assert_eq!(total, 6738);
    }
}
