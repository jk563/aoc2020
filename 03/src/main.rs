use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str;


fn read_to_struct<T: FromStr>(file_name: &str) -> Result<Vec<T>, T::Err> {
    std::fs::read_to_string(file_name).expect("Opening file failed")
                                      .lines().map(|x| x.parse()).collect()
}

fn main() {
    let tree_map = read_to_struct::<TreeLine>("input.txt").expect("Error reading file");
    let layers = tree_map.len();
    let repeat = tree_map[0].line.len();

    for treeline in &tree_map {
        println!("{}", treeline);
    }

    let inputs = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    println!();

    let mut multiplied: u64 = 1;

    for (step_x, step_y) in inputs {
        let mut trees_hit = 0;
        let mut x = 0;
        let mut y = 0;

        while y < layers {
            if let '#' = tree_map[y].line[x] {
                trees_hit += 1;
            }

            x = (x + step_x) % repeat;
            y += step_y;
        }
        
        multiplied *= trees_hit;

        println!("x step: {}, y step: {}", step_x, step_y);
        println!("Trees hit: {}", trees_hit);
        println!("Multiplied: {}", multiplied);
        println!();
    }
}

struct TreeLine {
    line: Vec<char>
}

impl Display for TreeLine {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let tree = str::from_utf8(&[240, 159, 142, 132]).unwrap();
        let white_box = str::from_utf8(&[226, 172, 156]).unwrap();
        let mut display_line = String::new();
        for b in &self.line {
            match b {
                '.' => display_line.push_str(white_box),
                '#' => display_line.push_str(tree),
                _ => panic!("Unknown character")
            }
        }
        println!("{}", display_line);
        Ok(())
    }
}

#[derive(Debug)]
struct TreeLineParseError;

impl FromStr for TreeLine {
    type Err = TreeLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TreeLine {
            line: s.chars().collect()
        })
    }
}
