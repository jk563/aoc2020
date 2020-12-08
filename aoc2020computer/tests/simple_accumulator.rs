use aoc2020computer::Computer;

#[test]
fn test_acc_addition() {
    let mut computer = Computer::new();
    let instructions = vec!["acc +5"];

    computer.load(&instructions);
    computer.run();

    assert_eq!(computer.get_accumulator(), 5);
}

#[test]
fn test_acc_subtraction() {
    let mut computer = Computer::new();
    let instructions = vec!["acc -5"];

    computer.load(&instructions);
    computer.run();

    assert_eq!(computer.get_accumulator(), -5);
}

#[test]
fn test_acc_multiple_accs() {
    let mut computer = Computer::new();
    let instructions = vec!["acc -5", "acc +7"];

    computer.load(&instructions);
    computer.run();

    assert_eq!(computer.get_accumulator(), 2);
}
