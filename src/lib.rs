pub fn arithmetic(table: Vec<Vec<i64>>, operators: Vec<char>) -> i64 {
    // Perform calculations
    let calculations = table.iter().enumerate().map(|(index, row)| match operators[index] {
        '+' => row.iter().sum::<i64>(),
        '*' => row.iter().product::<i64>(),
        _   => 0
    }).collect::<Vec<i64>>();

    println!("{:?}", calculations);

    calculations.iter().sum()
}

pub fn parse_input(contents: String) -> Vec<Vec<i64>> {
    let mut rows: Vec<_> = contents.split('\n')
        .filter(|row| !row.is_empty())
        .collect();

    // Remove last row of operators
    rows.pop();

    // Transpose each char column to row
    let count_columns = rows[0].len();
    let transpose = (0..count_columns)
        .map(|index| rows.iter()
            .map(move |row| row.chars().nth(index).expect("This is why I should trust no one."))
            .collect::<String>()
        ).map(|row| row.trim().to_string())
        .rev()
        .collect::<Vec<String>>();

    // Split at blank column (per instructions)
    transpose.split(|row| row.is_empty()).collect::<Vec<_>>()
        .iter().map(|&row|
            row.iter().map(|value| value.parse::<i64>().expect("Could not parse number."))
            .collect::<Vec<i64>>()
        ).collect::<Vec<Vec<i64>>>()
}

pub fn parse_input_operators(contents: String) -> Vec<char> {
    contents.split('\n')
        .filter(|row| !row.is_empty())
        .last()
        .expect("Could not parse last input line with operators.")
        .split_whitespace()
        .map(|value| match value {
            "+" => '+',
            "-" => '-',
            "*" => '*',
            "/" => '/',
            _   => ' ',
        })
        .rev()
        .collect::<Vec<char>>()
}
