use crate::AoCDay;

pub struct Code;

#[inline(always)]
fn get_counts(input: &str) -> Vec<(u64, u64)> {
    let mut counts = Vec::with_capacity(12);
    for _ in 0..12 {
        counts.push((0, 0));
    }
    for line in input.lines() {
        for (index, digit) in line.chars().enumerate() {
            if digit == '0' {
                counts[index].0 += 1;
            } else {
                counts[index].1 += 1;
            }
        }
    }
    counts
}

#[inline(always)]
fn calculate_rating(input: &[&str], counts: &[(u64, u64)], lt: bool, gt: bool, eq: bool) -> u64 {
    let mut found = false;
    let mut current_index = 0;
    let mut input = input.to_vec();
    let mut counts = counts.to_vec();

    while !found {
        let mut len = input.len();
        let mut next_counts = counts.to_vec();
        input = input
            .into_iter()
            .filter(|line| {
                let char = line.chars().nth(current_index).unwrap();
                let (zero_count, one_count) = counts[current_index];
                let filter_pass = if zero_count > one_count {
                    if char == '0' {
                        lt
                    } else {
                        !lt
                    }
                } else if zero_count == one_count {
                    if char == '0' {
                        eq
                    } else {
                        !eq
                    }
                } else if char == '0' {
                    gt
                } else {
                    !gt
                };
                if !filter_pass && len > 1 {
                    len -= 1;
                    for (index, char) in line.chars().enumerate() {
                        if char == '0' {
                            next_counts[index].0 -= 1;
                        } else {
                            next_counts[index].1 -= 1;
                        }
                    }
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();
        if input.len() <= 1 {
            found = true
        }
        current_index += 1;
        counts = next_counts;
    }
    u64::from_str_radix(input[0], 2).unwrap()
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let counts = get_counts(input);
        let gamma = counts
            .iter()
            .map(|x| if x.0 < x.1 { "0" } else { "1" })
            .collect::<String>();
        let gamma = i32::from_str_radix(&gamma, 2).unwrap();

        let epsilon = counts
            .iter()
            .map(|x| if x.0 > x.1 { "0" } else { "1" })
            .collect::<String>();
        let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

        let answer = gamma * epsilon;
        debug_assert_eq!(answer, 2954600);
        answer.to_string() // 2954600/~110μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let counts = get_counts(input);

        let input = input.lines().collect::<Vec<_>>();

        let o2 = calculate_rating(&input, &counts, true, false, false);
        let co2 = calculate_rating(&input, &counts, false, true, true);

        let answer = co2 * o2;
        debug_assert_eq!(answer, 1662846);
        format!("{}", co2 * o2) // 1662846/~370μs
    }

    fn both(&self, input: &str, extra_args: &[String]) -> String {
        let p1 = self.part1(input, extra_args);
        let p2 = self.part2(input, extra_args);
        format!(
            "Part1: {}\n\
            Part2: {}",
            p1, p2
        )
    }
}
