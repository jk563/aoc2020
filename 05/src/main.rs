use std::str::FromStr;

fn main() {
    let mut input: Vec<Seat> = std::fs::read_to_string("input.txt").expect("Opening file failed")
                                      .lines().map(|x| x.parse().unwrap()).collect();
    input.sort_by(|a, b| a.id.cmp(&b.id));

    println!("Highest Seat ID: {}", &input.get(input.len()-1).unwrap().id);
    
    for (i, seat) in input.iter().enumerate() {
        if i == 0 { continue };
        if seat.id != &input[i-1].id + 1 { println!("Missing Seat at: {}", seat.id - 1);}
    }
}

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
    id: usize,
}

impl FromStr for Seat {
    type Err = SeatParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = to_int(&s[0..7]);
        let column = to_int(&s[7..10]); 
        Ok(Seat { 
            row,
            column,
            id: (row * 8) + column
        })
    }
}

#[derive(Debug)]
struct SeatParseErr;

fn to_int(s: &str) -> usize {
    usize::from_str_radix(&s.chars()
                           .map(|x| if (x == 'B') || (x == 'R') {"1"} else {"0"} )
                           .collect::<Vec<&str>>()
                           .join(""),
                          2).unwrap()
}
