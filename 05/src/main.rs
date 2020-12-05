fn main() {
    let mut input: Vec<usize> = std::fs::read_to_string("input.txt")
        .expect("Opening file failed")
        .lines()
        .map(|x| {
            usize::from_str_radix(
                &x.chars()
                    .map(|x| match x {
                        'B' | 'R' => "1",
                        _ => "0",
                    })
                    .collect::<Vec<&str>>()
                    .join(""),
                2,
            )
            .unwrap()
        })
        .collect();

    input.sort();

    let highest = *input.last().unwrap();
    let lowest = *input.first().unwrap();
    println!("Highest Seat ID: {}", highest);

    println!(
        "Missing Seat at: {}",
        highest * (highest + 1) / 2 - input.iter().sum::<usize>() - ((1..lowest).sum::<usize>())
    );
}
