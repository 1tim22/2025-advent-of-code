// Recursion is king
// The basic algorithm is to find the greatest value in a slice of the bank.
// Then, recursively slice to the right of the maximum and find that maximum.
// Finally, sum all of them together
pub fn calculate_joltage(banks: Vec<Vec<u64>>, size: usize) -> u64 {
    let bank_iter = banks.iter().map(|bank| {
        let pairs: Vec<(usize, &u64)> = bank.iter().enumerate().collect();

        let mut list = slice_max(&pairs[..], size);

        // Sort by the original index
        list.sort_by(|(i0, _), (i1, _)| i1.cmp(i0));

        // Add number together for joltage
        let number = list.iter().map(|&(_, value)| value).enumerate().fold(0, |acc, (index, value)| {
            value * 10u64.pow((index) as u32) + acc
        });

        number
    }).inspect(|number| println!("{:?}", number));

    // Sum all of the joltages for all of the battery banks
    bank_iter.sum()
}

pub fn slice_max<'a>(slice: &'a [(usize, &u64)], halt: usize) -> Vec<(usize, &'a u64)> {
    // Identify maximum value in iterable
    let (index_max, pair_max) = slice.iter()
        .enumerate()
        .max_by(|(_, (i0, a)), (_, (i1, b))|
            if a == b {
                // Choose the left-most maximum
                i1.cmp(i0)
            } else {
                a.cmp(b)
            }
        ).unwrap_or((0, &(0, &0)));

    let mut list = if pair_max.1 > &0 {
        vec![*pair_max]
    } else {
        vec![]
    };

    if slice.len() > 0 && list.len() < halt {
        let (_, right) = slice.split_at(index_max + 1);
        let (left, _)  = slice.split_at(index_max);

        if list.len() < halt {
            list.extend(slice_max(right, halt - list.len()));
        }

        if list.len() < halt {
            list.extend(slice_max(left,  halt - list.len()));
        }
    }

    list
}

pub fn parse_input(contents: String) -> Vec<Vec<u64>> {
    contents.split_whitespace()
        .map(|row| row.chars().map(|value|
            value.to_digit(10).expect("Could not convert to base-10 digit.") as u64
        ).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>()
}
