use aoc2020computer::Computer;

#[test]
fn test_day08_part1_example() {
    let mut computer = Computer::new();
    let instructions = vec![
                            "nop +0",
                            "acc +1",
                            "jmp +4",
                            "acc +3",
                            "jmp -3",
                            "acc -99",
                            "acc +1",
                            "jmp -4",
                            "acc +6"
                        ];

    computer.load(&instructions);
    computer.run();
    
    assert_eq!(computer.get_accumulator(), 5);
}
