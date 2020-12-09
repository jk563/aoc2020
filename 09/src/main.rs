fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let input: Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    let preamble_size = 25;

    let invalid = input
        .iter()
        .enumerate()
        .skip(preamble_size)
        .filter(|&(i, x)| !valid(*x, &input[i - preamble_size..i]))
        .map(|(_, x)| *x)
        .collect::<Vec<usize>>()[0];

    println!("Invalid numbers: {}", invalid);

    for i in 0..input.len() {
        let mut total = input[i];
        for j in i + 1..input.len() {
            total += input[j];
            if total > invalid {
                break;
            }
            if total == invalid {
                println!(
                    "Encryption weakness: {}",
                    input[i..=j].iter().min().unwrap() + input[i..=j].iter().max().unwrap()
                );
            }
        }
    }
}

fn valid(x: usize, previous: &[usize]) -> bool {
    let mut low_to_high = previous.to_vec();
    low_to_high.sort();

    let mut larger_index = low_to_high.len() - 1;

    for (smaller_index, small_number) in low_to_high.iter().enumerate() {
        while small_number + low_to_high[larger_index] >= x {
            if small_number + low_to_high[larger_index] == x {
                return true;
            } else if larger_index == smaller_index + 1 {
                return false;
            }
            larger_index -= 1;
        }
    }
    false
}
