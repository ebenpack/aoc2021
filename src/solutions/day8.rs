use std::iter::FromIterator;

use crate::AoCDay;

use hashbrown::HashMap;
use hashbrown::HashSet;

pub struct Code;

fn normalize_signal(signal: &str) -> String {
    let mut signal = signal.chars().collect::<Vec<_>>();
    signal.sort_unstable();
    signal.iter().collect::<String>()
}

fn digit_to_set(digit: &str) -> HashSet<char> {
    let chars = digit.chars().collect::<Vec<_>>();
    HashSet::from_iter(chars)
}

struct DigitMap {
    mappings: HashMap<String, i32>,
    reverse_mappings: HashMap<i32, String>,
}

impl DigitMap {
    fn new() -> Self {
        DigitMap {
            mappings: HashMap::new(),
            reverse_mappings: HashMap::new(),
        }
    }
    fn insert(&mut self, k: i32, v: String) {
        self.mappings.insert(v.to_string(), k);
        self.reverse_mappings.insert(k, v);
    }
    fn get(&self, k: i32) -> Option<&String> {
        self.reverse_mappings.get(&k)
    }
    fn get_digit(&self, k: &str) -> Option<&i32> {
        self.mappings.get(k)
    }
    fn clear(&mut self) {
        self.mappings.clear();
        self.reverse_mappings.clear();
    }
    fn len(&mut self) -> usize {
        self.mappings.len()
    }
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut count = 0;
        for line in input.lines() {
            let input = line.split(" | ").collect::<Vec<_>>();
            let output = input[1].split(' ').collect::<Vec<_>>();
            count += output
                .iter()
                .filter(|x| matches!(x.len(), 2 | 4 | 3 | 7))
                .count();
        }
        debug_assert_eq!(count, 303);
        format!("{}", count) // 303/~175μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut total = 0;
        let mut mapping = DigitMap::new();
        for line in input.lines() {
            mapping.clear();
            let _input = line.split(" | ").collect::<Vec<_>>();
            let inputs = _input[0]
                .split(' ')
                .map(normalize_signal)
                .collect::<Vec<_>>();
            let outputs = _input[1]
                .split(' ')
                .map(normalize_signal)
                .collect::<Vec<_>>();

            let one = inputs.iter().find(|x| x.len() == 2);
            if let Some(one) = one {
                mapping.insert(1, one.to_string());
            }

            let four = inputs.iter().find(|x| x.len() == 4);
            if let Some(four) = four {
                mapping.insert(4, four.to_string());
            }

            let seven = inputs.iter().find(|x| x.len() == 3);
            if let Some(seven) = seven {
                mapping.insert(7, seven.to_string());
            }

            let eight = inputs.iter().find(|x| x.len() == 7);
            if let Some(eight) = eight {
                mapping.insert(8, eight.to_string());
            }

            // 0, 6, 9
            for digit in inputs.iter().filter(|x| x.len() == 6) {
                let digit = normalize_signal(digit);
                if let Some(seven) = mapping.get(7) {
                    // gotta be six
                    if !digit_to_set(seven).is_subset(&digit_to_set(&digit)) {
                        mapping.insert(6, digit.to_string());
                    }
                }
                if let Some(four) = mapping.get(4) {
                    // gotta be nine
                    if digit_to_set(four).is_subset(&digit_to_set(&digit)) {
                        mapping.insert(9, digit.to_string());
                    }
                }
                if mapping.get(6) != Some(&digit.to_string())
                    && mapping.get(9) != Some(&digit.to_string())
                {
                    // think this has to be 0?
                    mapping.insert(0, digit);
                }
                if mapping.len() == 7 {
                    break;
                }
            }

            // 2, 3, 5
            for digit in inputs.iter().filter(|x| x.len() == 5) {
                let digit = normalize_signal(digit);
                if let Some(seven) = mapping.get(7) {
                    // gotta be three
                    if digit_to_set(seven).is_subset(&digit_to_set(&digit)) {
                        mapping.insert(3, digit.to_string());
                    }
                }
                if let Some(nine) = mapping.get(9) {
                    // gotta be five
                    if digit_to_set(&digit).is_subset(&digit_to_set(nine))
                        && mapping.get(3) != Some(&digit.to_string())
                    {
                        mapping.insert(5, digit.to_string());
                    }
                }
                if mapping.get(3) != Some(&digit.to_string())
                    && mapping.get(5) != Some(&digit.to_string())
                {
                    // think this has to be 2?
                    mapping.insert(2, digit);
                }
                if mapping.len() == 10 {
                    break;
                }
            }
            let mut thingy = vec![];
            for digit in outputs {
                if let Some(n) = mapping.get_digit(&normalize_signal(&digit)) {
                    thingy.push(n.to_string());
                }
            }
            total += thingy.join("").parse::<i32>().unwrap();
        }
        debug_assert_eq!(total, 961734);
        format!("{:?}", total) // 961734/~5600μs
    }
}
