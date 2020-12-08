use aoc2020computer::Computer;

fn main() {
    let mut computer = Computer::new();
    let input = std::fs::read_to_string("input.txt").expect("File opening failed");
    let instructions = input.lines().collect();

    computer.load(&instructions);
    computer.run();

    println!("Accumulator on first infitine loop: {}", computer.get_accumulator());
}
