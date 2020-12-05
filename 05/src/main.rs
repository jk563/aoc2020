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

    println!("Highest Seat ID: {}", &input.last().unwrap());

    for (i, id) in input.iter().enumerate() {
        if i == 0 {
            continue;
        };
        if *id != input[i - 1] + 1 {
            println!("Missing Seat at: {}", id - 1);
            break;
        }
    }
}
