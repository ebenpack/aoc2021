use crate::AoCDay;

use hashbrown::HashMap;
use hashbrown::HashSet;

use std::hash::Hash;
use std::hash::Hasher;

pub struct Code;

struct HashWrapper<T>(HashSet<T>);

impl<T> PartialEq for HashWrapper<T>
where
    T: Eq + Hash + Clone,
{
    #[inline(always)]
    fn eq(&self, other: &HashWrapper<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for HashWrapper<T> where T: Eq + Hash + Clone {}

impl<T> Hash for HashWrapper<T>
where
    T: Eq + Hash + Clone,
{
    #[inline(always)]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for x in &self.0 {
            x.hash(state);
        }
    }
}

impl<T> HashWrapper<T>
where
    T: Eq + Hash + Clone,
{
    #[inline(always)]
    fn new() -> Self {
        HashWrapper(HashSet::new())
    }
    #[inline(always)]
    fn contains(&self, key: &T) -> bool {
        self.0.contains(key)
    }
    #[inline(always)]
    fn insert(&mut self, key: T) -> bool {
        self.0.insert(key)
    }
    #[inline(always)]
    fn clone(&self) -> Self {
        HashWrapper(self.0.clone())
    }
}

#[inline(always)]
fn solve(
    map: &HashMap<String, Vec<String>>,
    seent: &HashWrapper<String>,
    current: &str,
    visited_twice: bool,
    memo: &mut HashMap<(String, bool, HashWrapper<String>), i32>,
) -> i32 {
    if let Some(n) = memo.get(&(current.to_string(), visited_twice, seent.clone())) {
        return *n;
    }
    let original_visited_twice = visited_twice;
    let mut visited_twice = visited_twice;
    if current == "end" {
        memo.insert((current.to_string(), visited_twice, seent.clone()), 1);
        return 1;
    } else if seent.contains(&current.to_string()) {
        if visited_twice || current == "start" {
            memo.insert((current.to_string(), visited_twice, seent.clone()), 0);
            return 0;
        } else {
            visited_twice = true;
        }
    }

    let answer = map
        .get(current)
        .unwrap()
        .iter()
        .map(|cave| {
            let mut new_seent = seent.clone();
            if current.chars().all(|c| c.is_lowercase()) {
                new_seent.insert(current.to_string());
            }
            let answer = solve(map, &new_seent, cave, visited_twice, memo);
            memo.insert((cave.to_string(), visited_twice, new_seent), answer);
            answer
        })
        .sum();
    memo.insert(
        (current.to_string(), original_visited_twice, seent.clone()),
        answer,
    );
    answer
}

#[inline(always)]
fn map_from_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let line = line.split('-').collect::<Vec<_>>();

        let entry = map.entry(line[0].to_string()).or_insert(vec![]);
        entry.push(line[1].to_string());

        let entry = map.entry(line[1].to_string()).or_insert(vec![]);
        entry.push(line[0].to_string());
    }
    map
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let map = map_from_input(input);
        let mut memo = HashMap::with_capacity(5000);
        let answer = solve(&map, &HashWrapper::new(), "start", true, &mut memo);

        debug_assert_eq!(answer, 5076);
        format!("{}", answer) // 5076/~5000μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let map = map_from_input(input);
        let mut memo = HashMap::with_capacity(5000);
        let answer = solve(&map, &HashWrapper::new(), "start", false, &mut memo);

        debug_assert_eq!(answer, 145643);
        format!("{}", answer) // 145643/~20000μs
    }
}
