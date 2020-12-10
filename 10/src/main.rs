use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading file");
    let mut input: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    input.push(0);
    input.sort();

    let mut one = 0;
    let mut three = 1;

    let mut previous = &input[0];

    for adapter in input.iter().skip(1) {
        match adapter - previous {
            3 => three += 1,
            1 => one += 1,
            2 => (),
            _ => panic!("Difference in joltage unexpected"),
        }
        previous = adapter;
    }

    println!("1 jolt * 3 jolt differences = {}", one * three);

    input.reverse();

    let mut last_3 = HashMap::new();
    last_3.insert(input[0] + 3, 1_u64);

    for element in input {
        last_3.retain(|k, _| *k <= element + 3);
        last_3.insert(element, last_3.values().sum());
    }

    println!("Combinations: {}", last_3.get(&0).unwrap());
}
