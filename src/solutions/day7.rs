use crate::AoCDay;

pub struct Code;

fn fuel_cost(x: i64, y: i64) -> i64 {
    let n = (x - y).abs();
    (n * (n + 1)) / 2
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut inputs = input
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        inputs.sort_unstable();
        let index = inputs.len() / 2;
        let median = inputs[index];
        let answer: i32 = inputs.iter().map(|num| (num - median).abs()).sum();
        debug_assert_eq!(answer, 347011);
        format!("{}", answer) // 347011/50μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let inputs = input
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mean = inputs.iter().sum::<i64>() / inputs.len() as i64;
        let answer: i64 = [mean - 1, mean, mean + 1]
            .iter()
            .map(|x| inputs.iter().map(|y| fuel_cost(*x, *y)).sum())
            .min()
            .unwrap();

        debug_assert_eq!(answer, 98363777);
        format!("{}", answer) // 98363777/~40μs
    }
}
