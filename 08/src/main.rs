use aoc2020computer::Computer;

fn main() {
    let mut computer = Computer::new();
    let input = std::fs::read_to_string("input.txt").expect("File opening failed");
    let instructions = input.lines().collect();

    computer.load(&instructions);
    computer.run();

    println!(
        "Accumulator on first infinite loop: {}",
        computer.get_accumulator()
    );

    for (i, instruction) in instructions.iter().enumerate() {
        if instruction.contains("jmp") || instruction.contains("nop") {
            let mut new_instructions = instructions.clone();
            let new_instruction = match &instruction[..3] {
                "jmp" => instruction.replace("jmp", "nop"),
                "nop" => instruction.replace("nop", "jmp"),
                _ => String::new(),
            };
            new_instructions.remove(i);
            new_instructions.insert(i, &new_instruction);

            computer.load(&new_instructions);
            let success = computer.run();
            if success {
                println!("Accumulator after fix: {}", computer.get_accumulator());
            }
        }
    }
}
