fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading file");
    let input: Vec<&str> = input.lines().collect();
    let mut heading = 90;
    let mut x = 0;
    let mut y = 0;

    for instruction in &input {
        let char_iterator = &mut instruction.chars();
        let direction = &char_iterator.next().unwrap().to_string()[..];
        let magnitude = char_iterator.collect::<String>().parse::<isize>().unwrap();

        match direction {
            "N" => y += magnitude,
            "S" => y -= magnitude,
            "E" => x += magnitude,
            "W" => x -= magnitude,
            "R" => heading = (heading + magnitude) % 360,
            "L" => heading = (360 + heading - magnitude) % 360,
            "F" => match heading {
                0 => y += magnitude,
                180 => y -= magnitude,
                90 => x += magnitude,
                270 => x -= magnitude,
                _ => panic!("Where am I going?"),
            },
            _ => panic!("What do you want me to do?"),
        }
    }

    println!("Manhatten distance: {}", x.abs() + y.abs());

    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    let mut x = 0;
    let mut y = 0;
    for instruction in &input {
        let char_iterator = &mut instruction.chars();
        let direction = &char_iterator.next().unwrap().to_string()[..];
        let magnitude = char_iterator.collect::<String>().parse::<isize>().unwrap();

        match direction {
            "N" => waypoint_y += magnitude,
            "S" => waypoint_y -= magnitude,
            "E" => waypoint_x += magnitude,
            "W" => waypoint_x -= magnitude,
            "R" => {
                for _ in 0..(magnitude / 90) {
                    let curr_waypoint_x = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -curr_waypoint_x;
                }
            }
            "L" => {
                for _ in 0..(magnitude / 90) {
                    let curr_waypoint_x = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = curr_waypoint_x;
                }
            }
            "F" => {
                y += magnitude * waypoint_y;
                x += magnitude * waypoint_x;
            }
            _ => panic!("What do you want me to do?"),
        }
    }

    println!("Manhatten distance: {}", x.abs() + y.abs());
}
