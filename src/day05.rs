use crate::computer::Computer;

#[aoc(day5, part1)]
fn part_1(input: &str) -> i32 {
    let mut computer = Computer::from(input);
    computer.input = Some(1);
    computer.run_program().expect("Program failed");
    computer.check_diagnostics().unwrap()
}

#[aoc(day5, part2)]
fn part_2(input: &str) -> i32 {
    Computer::run_from(input, Some(5)).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::computer::{Computer, OpCode, ParameterMode};

    #[test]
    fn test_save_ouput_command() {
        let input = "3,0,4,0,99";
        let mut computer = Computer::from(input);
        computer.input = Some(1);
        let res = computer.run_program();
        assert_eq!(res, Ok(1));
    }

    #[test]
    fn read_command() {
        let input = "1002,0,10,3";
        let computer = Computer::from(input);
        assert_eq!(
            computer.read_command(),
            (
                OpCode::Mul,
                [
                    ParameterMode::Position,
                    ParameterMode::Immediate,
                    ParameterMode::Position
                ]
            )
        );
    }

    #[test]
    fn test_modes() {
        let mut computer = Computer::from("1002,4,3,4,33");
        let res = computer.run_program();
        assert_eq!(res, Ok(1002));
        assert_eq!(computer.memory, vec![1002, 4, 3, 4, 99]);

        let mut computer = Computer::from("1101,100,-1,4,0");
        computer.run_program().expect("Program failed");
        assert_eq!(computer.memory, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_jump() {
        // Equal to 8 (position mode)
        let sample_1 = "3,9,8,9,10,9,4,9,99,-1,8";

        let mut computer = Computer::from(sample_1);
        computer.input = Some(8);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(1));

        let mut computer = Computer::from(sample_1);
        computer.input = Some(0);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(0));

        // Less than 8 (position mode)
        let sample_2 = "3,9,7,9,10,9,4,9,99,-1,8";

        let mut computer = Computer::from(sample_2);
        computer.input = Some(7);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(1));

        let mut computer = Computer::from(sample_2);
        computer.input = Some(8);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(0));

        // Equal to 8 (immediate mode)
        let sample_3 = "3,3,1108,-1,8,3,4,3,99";

        let mut computer = Computer::from(sample_3);
        computer.input = Some(8);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(1));

        let mut computer = Computer::from(sample_3);
        computer.input = Some(0);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(0));

        // Less than 8 (immediate mode)
        let sample_4 = "3,3,1107,-1,8,3,4,3,99";

        let mut computer = Computer::from(sample_4);
        computer.input = Some(7);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(1));

        let mut computer = Computer::from(sample_4);
        computer.input = Some(8);
        let res = computer.run_program();
        assert!(res.is_ok());
        assert_eq!(computer.check_diagnostics(), Ok(0));

        fn non_zero(input: i32) -> Result<i32, ()> {
            let mut computer = Computer::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
            computer.input = Some(input);
            let res = computer.run_program();
            assert!(res.is_ok());
            computer.check_diagnostics().or(Err(()))
        }

        assert_eq!(non_zero(0), Ok(0));
        assert_eq!(non_zero(10), Ok(1));

        assert_eq!(
            Computer::run_from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", Some(0)),
            Ok(0)
        );
        assert_eq!(
            Computer::run_from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", Some(2)),
            Ok(1)
        );

        let larger_program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(Computer::run_from(larger_program, Some(0)), Ok(999));
        assert_eq!(Computer::run_from(larger_program, Some(8)), Ok(1000));
        assert_eq!(Computer::run_from(larger_program, Some(9)), Ok(1001));
    }
}
