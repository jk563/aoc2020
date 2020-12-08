use aoc2020computer::Computer;

#[test]
fn test_jump() {
    let mut computer = Computer::new();
    let instructions = vec!["jmp +4", "acc +3", "acc +2", "jmp +2", "jmp -2"];

    computer.load(&instructions);
    computer.run();

    assert_eq!(computer.get_accumulator(), 2);
}
