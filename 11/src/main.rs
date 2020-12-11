fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading to string");
    let input: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_string()).collect())
        .collect();

    let mut changes = true;
    let mut part1_state = input.clone();

    while changes {
        changes = false;

        let mut new_state = vec![];
        for (y, row) in part1_state.iter().enumerate() {
            let mut new_row = vec![];
            for (x, _) in row.iter().enumerate() {
                new_row.insert(x, update_seat(x, y, &part1_state));
            }
            if new_row != part1_state[y] {
                changes = true;
            }
            new_state.insert(y, new_row);
        }
        part1_state = new_state;
    }

    println!("Occupied Seats Part 1: {}", occupied_seats(&part1_state));

    let mut part2_state = input.clone();
    changes = true;
    while changes {
        changes = false;
        let mut new_state = vec![];
        for (y, row) in part2_state.iter().enumerate() {
            let mut new_row = vec![];
            for (x, _) in row.iter().enumerate() {
                new_row.insert(x, update_v2(x, y, &part2_state));
            }
            if new_row != part2_state[y] {
                changes = true;
            }
            new_state.insert(y, new_row);
        }
        part2_state = new_state;
    }

    println!("Occupied Seats Part 2: {}", occupied_seats(&part2_state));
}

fn occupied_seats(grid: &Vec<Vec<String>>) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|seat| seat == &"#").count())
        .sum()
}

fn update_v2(seat_x: usize, seat_y: usize, grid: &Vec<Vec<String>>) -> String {
    let seat = &grid[seat_y][seat_x];
    if seat == "." {
        return String::from(".");
    }
    let mut occupied = 0;

    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            if direction_has_seat(seat_x, seat_y, x, y, &grid) {
                occupied += 1;
            }
        }
    }

    if seat == "L" && occupied == 0 {
        return String::from("#");
    }

    if seat == "#" && occupied >= 5 {
        return String::from("L");
    }

    String::from(seat)
}

fn direction_has_seat(
    seat_x: usize,
    seat_y: usize,
    step_x: isize,
    step_y: isize,
    grid: &Vec<Vec<String>>,
) -> bool {
    let mut x = seat_x as isize;
    let mut y = seat_y as isize;

    while (y >= 0 && y < grid.len() as isize) && (x >= 0 && x < grid[0].len() as isize) {
        if !(x == seat_x as isize && y == seat_y as isize) {
            let thing = &grid[y as usize][x as usize];
            match &thing[..] {
                "#" => return true,
                "L" => return false,
                "." => (),
                _ => panic!("What happened here?"),
            }
        }
        x += step_x;
        y += step_y;
    }

    false
}

fn update_seat(seat_x: usize, seat_y: usize, grid: &Vec<Vec<String>>) -> String {
    let seat = &grid[seat_y][seat_x];
    if seat == "." {
        return String::from(".");
    }

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut occupied_seats = 0;
    for other_y in (seat_y as isize - 1)..=(seat_y as isize + 1) {
        if other_y < 0 || other_y >= grid_height as isize {
            continue;
        }
        for other_x in (seat_x as isize - 1)..=(seat_x as isize + 1) {
            if (other_x < 0 || other_x >= grid_width as isize)
                || (other_x == seat_x as isize && other_y == seat_y as isize)
            {
                continue;
            }
            if &grid[other_y as usize][other_x as usize] == "#" {
                occupied_seats += 1;
            }
        }
    }

    if seat == "L" && occupied_seats == 0 {
        return String::from("#");
    }

    if seat == "#" && occupied_seats >= 4 {
        return String::from("L");
    }

    return String::from(seat);
}
