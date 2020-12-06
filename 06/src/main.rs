use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Opening file failed");

    let groups: Vec<&str> = input.split("\n\n").collect();

    let any_sum: usize = groups.iter().map(|x| any_yes(x)).sum();
    let all_sum: usize = groups.iter().map(|x| all_yes(x)).sum();

    println!("{}", any_sum);
    println!("{}", all_sum);
}

fn any_yes(group: &str) -> usize {
    let mut set = HashSet::new();
    for c in group
        .chars()
        .filter(|d| !d.is_whitespace())
        .collect::<Vec<char>>()
    {
        set.insert(c);
    }
    set.len()
}

fn all_yes(group: &str) -> usize {
    let individuals: Vec<HashSet<char>> = group
        .lines()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect();
    if individuals.len() == 0 {
        return 0;
    }
    let mut all_yes = individuals[0].clone();
    for individual in individuals {
        all_yes = all_yes.intersection(&individual).map(|c| *c).collect();
    }
    all_yes.len()
}
