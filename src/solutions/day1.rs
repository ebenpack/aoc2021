use crate::AoCDay;

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_argss: &[String]) -> String {
        let mut previous = None;
        let mut increases: u64 = 0;
        for line in input.lines() {
            let current = line.trim().parse::<u64>().unwrap();

            match previous {
                None => (),
                Some(previous) => {
                    if current > previous {
                        increases += 1;
                    }
                }
            }
            previous = Some(current);
        }
        debug_assert_eq!(increases, 1316);
        increases.to_string() // 1316/~60μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let lines = input
            .lines()
            .map(|line| line.trim().parse::<u64>().unwrap())
            .take(3)
            .collect::<Vec<_>>();

        // Basically, keep a rolling window of the past three numbers,
        // kinda like a ring buffer. `window_index` tracks the oldest
        // entry, allowing us to swap in the newest entry. This is
        // a bit verbose, but faster than a
        let mut increases: u64 = 0;
        let mut window_index = 0;
        let mut window = [lines[0], lines[1], lines[2]];
        let mut window_total = lines[0] + lines[1] + lines[2];
        for line in input.lines().skip(3) {
            let num = line.trim().parse::<u64>().unwrap();

            let current_total = window_total - window[window_index] + num;
            if current_total > window_total {
                increases += 1;
            }
            window[window_index] = num;
            window_index += 1;
            window_index %= 3;
            window_total = current_total;
        }

        debug_assert_eq!(increases, 1344);
        increases.to_string() // 1344/~65μs
    }
}
