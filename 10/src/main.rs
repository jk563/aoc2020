use std::time::Instant;

use std::collections::HashMap;

fn main() {
    let before_load = Instant::now();
    let input = std::fs::read_to_string("input.txt").expect("Error reading file");
    let mut input: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let after_load = Instant::now();
    input.push(0);
    input.sort();

    let after_sort = Instant::now();
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
    let after_pt_1 = Instant::now();

    input.reverse();

    let mut last_3 = HashMap::new();
    last_3.insert(input[0] + 3, 1_u64);

    for element in input {
        last_3.retain(|k, _| *k <= element + 3);
        last_3.insert(element, last_3.values().sum());
    }

    println!("Combinations: {}", last_3.get(&0).unwrap());
    let after_pt_2 = Instant::now();
    println!("Load time: {:?}", after_load.duration_since(before_load));
    println!("Sort time: {:?}", after_sort.duration_since(after_load));
    println!("Pt 1 time: {:?}", after_pt_1.duration_since(after_sort));
    println!("Pt 2 time: {:?}", after_pt_2.duration_since(after_pt_1));
}
