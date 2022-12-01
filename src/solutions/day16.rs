use crate::computer::Computer;
use crate::AoCDay;

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut compy = Computer::load_input(input);
        let packet = compy.parse_packet();
        let version_numbers = packet.version_numbers();
        let answer = version_numbers.iter().sum::<i64>();

        debug_assert_eq!(answer, 1012);
        format!("{}", answer) // 1012/~980μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut compy = Computer::load_input(input);
        let packet = compy.parse_packet();

        let answer = packet.evaluate();

        debug_assert_eq!(answer, 2223947372407);
        format!("{}", answer) // 2223947372407/~920μs
    }
}
