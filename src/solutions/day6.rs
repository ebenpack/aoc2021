use crate::AoCDay;

pub struct Code;

#[inline(always)]
fn count_the_fishies(input: &str, days: i32) -> u64 {
    let mut the_fishies = [0; 9];
    for num_days in input.split(',').map(|num| num.parse::<usize>().unwrap()) {
        the_fishies[num_days] += 1;
    }

    for _ in 0..days {
        let zero = the_fishies[0];
        the_fishies[0] = 0;
        the_fishies.rotate_left(1);
        the_fishies[6] += zero;
        the_fishies[8] = zero;
    }

    the_fishies.iter().sum()
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let total = count_the_fishies(input, 80);
        debug_assert_eq!(total, 386640);
        format!("{}", total) // 386640/12μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let total = count_the_fishies(input, 256);
        debug_assert_eq!(total, 1733403626279);
        format!("{}", total) // 1733403626279/13μs
    }
}
