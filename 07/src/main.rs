use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut node_parents: HashMap<String, HashSet<String>> = HashMap::new();
    let mut node_children: HashMap<String, HashMap<String, usize>> = HashMap::new();

    let input: String = std::fs::read_to_string("input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let line_components: Vec<&str> = line.split("contain").collect();
        let parent_bag_type = line_components[0].split(" ").collect::<Vec<&str>>()[0..=1].join(" ");
        let mut children_nodes: HashMap<String, usize> = HashMap::new();
        for child in line_components[1].trim().split(",").collect::<Vec<&str>>() {
            let child_components = child.trim().split(" ").collect::<Vec<&str>>();
            let amount = child_components[0];
            if amount == "no" {
                break;
            }
            let bag_type = child_components[1..3].join(" ");
            node_parents
                .entry(bag_type.clone())
                .or_insert(HashSet::new());
            node_parents
                .get_mut(&bag_type)
                .unwrap()
                .insert(parent_bag_type.clone());
            children_nodes.insert(bag_type, amount.parse().unwrap());
        }
        node_children.insert(parent_bag_type, children_nodes);
    }

    let mut sized_bags: HashMap<String, usize> = HashMap::new();

    while sized_bags.len() < node_parents.len() || !sized_bags.contains_key("shiny gold") {
        for (bag_type, bag_children) in &node_children {
            if sized_bags.contains_key(bag_type) {
                continue;
            }
            if node_children.get(bag_type).unwrap().len() == 0 {
                sized_bags.insert(bag_type.clone(), 1);
                continue;
            }
            if bag_children.len()
                == bag_children
                    .keys()
                    .filter(|x| sized_bags.contains_key(&x.to_string()))
                    .count()
            {
                sized_bags.insert(
                    bag_type.clone(),
                    bag_children
                        .iter()
                        .map(|(key, value)| sized_bags.get(key).unwrap() * value)
                        .sum::<usize>()
                        + 1,
                );
            }
        }
    }

    println!(
        "Bag colours containing shiny gold: {}",
        contained_by("shiny gold", &node_parents).len()
    );
    println!(
        "Bags required in shiny gold: {}",
        sized_bags.get("shiny gold").unwrap() - 1
    );
}

fn contained_by(bag_type: &str, node_parents: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut result_set = HashSet::new();
    let mut parents = match node_parents.get(bag_type) {
        Some(x) => x.into_iter().collect::<Vec<&String>>(),
        None => vec![],
    };
    while parents.len() != 0 {
        let parent = parents.pop().unwrap();
        result_set.insert(parent);
        if node_parents.contains_key(parent) {
            parents.append(
                &mut node_parents
                    .get(parent)
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<&String>>(),
            );
        }
    }
    result_set
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}
