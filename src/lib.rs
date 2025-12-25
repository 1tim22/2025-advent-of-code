// Basic algorithm
// Instead of working with a matrix of sorts, I decided to simply use a flat vector
// with clever offsets to represent adjacent paper rolls. For example, if the iterator
// is at index 7 in a row of length 10, then the adjacent areas would be calculated as
// `index - 11` (top-left), `index - 10` (top), `index - 9` (top-right), `index - 1`
// (left), `index + 1` (right), `index + 9` (bottom-left), `index + 10` (bottom),
// `index + 11` (bottom-right).
// Thus, all 8 possible adjacent locations must be calculated for each position. If the
// sum of all adjacent rolls is less than 4, the roll at the current index qualifies
// for access by a forklift.
// Part 2 made use of recursion!
pub fn window(table: Vec<Vec<u32>>) -> u32 {
    let count = table.first().expect("Could not parse table.").len() as i64;
    let list  = table.into_iter().flatten().collect::<Vec<u32>>();

    remove_rolls(list, count)
}

pub fn remove_rolls(list: Vec<u32>, count: i64) -> u32 {
    let accessible_list = list.iter().enumerate().map(|(index, &value)| {
        let relative_index = index as i64 % count;

        if 1 > value {
            value as u32
        } else {
            let offsets = &[      // Example offset:
                -1 * (count + 1), // -11
                -1 *  count,      // -10
                -1 * (count - 1), //  -9
                -1,               //  -1
                 1,               //   1
                count - 1,        //   9
                count,            //  10
                count + 1,        //  11
            ].map(|offset| {
                if (index as i64 + offset < 0)
                    // Ignore bogus offsets for start row index and end row index
                    || (relative_index < 1           && offset == (-1 *          1))
                    || (relative_index < 1           && offset ==       (count - 1))
                    || (relative_index > (count - 2) && offset == (-1 * (count - 1)))
                    || (relative_index > (count - 2) && offset ==       (count + 1))
                    || (relative_index > (count - 2) && offset ==                1)
                    || (relative_index < 1           && offset == (-1 * (count + 1)))
                {
                    0
                } else {
                    *list.get((index as i64 + offset) as usize).unwrap_or(&0)
                }
            });

            let adjacent_rolls = offsets.iter().sum::<u32>();

            if adjacent_rolls < 4 { 1 } else { 0 }
        }
    }).collect::<Vec<u32>>();

    // Recursively remove accessible paper rolls until no more rolls are removed
    let sum = accessible_list.iter().sum::<u32>();

    if 0 < sum {
        let new_list = list.iter().enumerate().map(|(index, &value)| {
            value - accessible_list[index]
        }).collect();

        sum + remove_rolls(new_list, count)
    } else {
        sum
    }
}

pub fn parse_input(contents: String) -> Vec<Vec<u32>> {
    contents.split_whitespace()
        .map(|row| row.chars()
            .map(|value| match value {
                '@' => 1,
                _   => 0
            }).collect()
        ).collect::<Vec<Vec<u32>>>()
}
