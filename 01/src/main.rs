use std::fs::File;
use std::io::{BufReader,BufRead};
use std::cmp::Ordering;

fn file_to_vec(filename: &str) -> std::io::Result<Vec<usize>> {
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);
    let mut data = Vec::new();
    for line in buffered.lines() {
        data.push(line.unwrap().parse::<usize>().expect("Flail"));
    }

    Ok(data)
}

fn main() {
    let open_expenses = file_to_vec("expenses.txt");

    match open_expenses {
        Ok(expenses) => do_the_thing(expenses),
        Err(e) => println!("{}", e),
    }
}

fn do_the_thing(mut expenses: Vec<usize>) {
    expenses.sort();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    let mut counter = 0;

    'outer: for (index_a, exp_a) in expenses.iter().enumerate() {
        for (index_b, exp_b) in expenses.iter().skip(index_a + 1).enumerate() {
            'inner: for exp_c in expenses.iter().skip(index_a + index_b + 1) {
                counter += 1;
                match 2020.cmp(&(exp_a + exp_b + exp_c)) {
                    Ordering::Greater => (),
                    Ordering::Equal => {
                        a = *exp_a;
                        b = *exp_b;
                        c = *exp_c;
                        break 'outer
                    },
                    Ordering::Less => break 'inner,
                }
            }
        }
    };

    println!("a={}, b={}, c={}", a, b, c);
    println!("Made {} comparisons", counter);

    println!("{} * {} * {} = {}", a, b, c, a * b * c);
}
