pub fn arithmetic(table: Vec<Vec<i64>>, operators: Vec<char>) -> i64 {
    let count_columns = table[0].len();

    // Convert columns to rows
    let transpose = (0..count_columns)
        .map(|index| table.iter().map(move |row| row[index])
            .collect::<Vec<i64>>()
        ).collect::<Vec<Vec<i64>>>();

    println!("transpose: {:?}", transpose);

    // Perform calculations
    let calculations = transpose.iter().enumerate().map(|(index, row)| match operators[index] {
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
    rows.pop();

    println!("{:?}", rows);

    rows.iter().map(|row| row.split_whitespace().map(|value|
        value.parse::<i64>().expect("Could not convert to base-10 digit.")
    ).collect::<Vec<i64>>())
    .collect::<Vec<Vec<i64>>>()
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
        .collect::<Vec<char>>()
}
