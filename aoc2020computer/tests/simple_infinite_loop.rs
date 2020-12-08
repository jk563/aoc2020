use aoc2020computer::Computer;

#[test]
fn test_infinite_loop() {
    let mut computer = Computer::new();
    let instructions = vec!["nop +0", "acc +3", "jmp -1"];
    
    computer.load(&instructions);
    let successful = computer.run();

    assert_eq!(computer.get_accumulator(), 3);
    assert_eq!(successful, false);
}
