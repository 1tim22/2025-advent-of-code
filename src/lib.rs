pub fn trace_tachyon(manifold: Vec<Vec<bool>>) -> u32 {
    let mut total_splits: u32 = 0;

    // The first row is the beginning of the tachyon beam
    let test = manifold.into_iter().reduce(|beam, manifold| {
        let (count, next) = beam_multiplier(beam, manifold);

        total_splits += count;

        next
    }).unwrap_or(vec![]);

    total_splits
}

pub fn beam_multiplier(beam: Vec<bool>, splitter: Vec<bool>) -> (u32, Vec<bool>) {
    let split_beam   = intersect(beam.clone(), splitter).expect("This is why I trust no one.");
    let mut beam_new = beam;

    let remove = (0..split_beam.len()).map(|index| {
        if split_beam[index] {
            if index > 0 {
                beam_new[index - 1] = true;
            }

            if index < (split_beam.len() - 1) {
                beam_new[index + 1] = true;
            }
        }

        split_beam[index]
    }).collect::<Vec<bool>>();

    (0..beam_new.len()).for_each(|index| if remove[index] {
        beam_new[index] = false;
    });

    println!("beam_new: {:?}",
        beam_new.iter().map(|value| match value { true => '|', _ => '.'}).collect::<Vec<char>>()
    );

    (split_beam.iter().map(|value| match value { true => 1, _ => 0}).sum(), beam_new)
}

pub fn parse_manifold(content: String) -> Vec<Vec<bool>> {
    content.split_whitespace()
        .map(|row| {
            row.chars().map(|value| match value {
                's' | 'S' | '^' => true,
                _               => false
            }).collect::<Vec<bool>>()
        }).filter(|row| row.iter().any(|&value| value))
        .collect::<Vec<Vec<bool>>>()
}

fn intersect(a: Vec<bool>, b: Vec<bool>) -> Result<Vec<bool>, &'static str> {
    if a.len() != b.len() {
        Err("Iterables which vary in size cannot be intersected.")
    } else {
        Ok((0..a.len()).map(|index| a[index] && b[index]).collect())
    }
}
