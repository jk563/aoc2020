use aoc2020computer::Computer;

#[test]
fn test_noop() {
    let mut computer = Computer::new();
    let instructions = vec!["nop +0"];

    computer.load(&instructions);
    computer.run();
}
