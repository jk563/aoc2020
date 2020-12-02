use std::str::FromStr;

fn read_to_struct<T: FromStr>(file_name: &str) -> Result<Vec<T>, T::Err> {
    std::fs::read_to_string(file_name).expect("Opening file failed")
                                      .lines().map(|x| x.parse()).collect()
}

fn main() {
    let things = match read_to_struct::<PolicyAndPassword>("input.txt") {
        Ok(x) => x,
        Err(_) => panic!("Failed to read file")
    };

    println!("{}", things.iter().filter(|x| x.policy_one()).count());
    println!("{}", things.iter().filter(|x| x.policy_two()).count());
}

struct PolicyAndPassword {
    first_position: usize,
    second_position: usize,
    required_letter: char,
    password: String
}

impl PolicyAndPassword {
    fn policy_one(&self) -> bool {
        let required_letter_count = self.password.matches(self.required_letter).count();

        (self.first_position <= required_letter_count) 
        && (required_letter_count <= self.second_position)
    }

    fn policy_two(&self) -> bool {
        (self.password.as_bytes()[self.first_position - 1] as char == self.required_letter)
        ^ (self.password.as_bytes()[self.second_position - 1] as char == self.required_letter)
    }
}

#[derive(Debug)]
pub struct PAPParseError;

impl From<std::num::ParseIntError> for PAPParseError {
    fn from(_: std::num::ParseIntError) -> PAPParseError {
        PAPParseError 
    }
}

impl FromStr for PolicyAndPassword {
    type Err = PAPParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(":").collect();
        let constraint: Vec<&str> = components.get(0).ok_or(PAPParseError)?.split(" ").collect();
        let position_numbers: Vec<&str> = constraint.get(0).ok_or(PAPParseError)?.split("-").collect();

        Ok(PolicyAndPassword {
            first_position: position_numbers.get(0).ok_or(PAPParseError)?.parse::<usize>()?,
            second_position: position_numbers.get(1).ok_or(PAPParseError)?.parse::<usize>()?,
            required_letter: char::from(*constraint.get(1).ok_or(PAPParseError)?.as_bytes().get(0).ok_or(PAPParseError)?),
            password: components.get(1).ok_or(PAPParseError)?.trim().to_string(),
        })
    }
}
