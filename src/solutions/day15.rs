use hashbrown::HashMap;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use crate::AoCDay;

pub struct Code;

type Coord = (usize, usize);

struct Mapping {
    map: HashMap<Coord, i32>,
    x_size: usize,
    y_size: usize,
}

#[derive(PartialEq, Eq)]
struct Node {
    coords: Coord,
    weight: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl Mapping {
    #[inline(always)]
    fn from_map(map: &HashMap<Coord, i32>, x_size: usize, y_size: usize) -> Self {
        let mut new_map = HashMap::new();
        for (coords, weight) in map {
            new_map.insert(*coords, *weight);
        }
        Mapping {
            map: new_map,
            x_size,
            y_size,
        }
    }
    #[inline(always)]
    fn neighbors(&self, x: usize, y: usize) -> Vec<(Coord, i32)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push(((x - 1, y), *self.map.get(&(x - 1, y)).unwrap()));
        }
        if y > 0 {
            neighbors.push(((x, y - 1), *self.map.get(&(x, y - 1)).unwrap()));
        }
        if x < self.x_size - 1 {
            neighbors.push(((x + 1, y), *self.map.get(&(x + 1, y)).unwrap()));
        }
        if y < self.y_size - 1 {
            neighbors.push(((x, y + 1), *self.map.get(&(x, y + 1)).unwrap()));
        }
        neighbors
    }
    #[inline(always)]
    fn solve(&self) -> i32 {
        let Mapping {
            map,
            x_size,
            y_size,
        } = self;
        let mut distances = HashMap::new();
        let mut unvisited = HashMap::new();
        let mut unvisited_heap = BinaryHeap::new();
        let total_size = x_size * y_size;
        let mut visited_size = 0;
        for ((x, y), _) in map.iter() {
            for ((n_x, n_y), weight) in self.neighbors(*x, *y) {
                let entry = distances.entry((*x, *y)).or_insert(vec![]);
                entry.push(((n_x, n_y), weight));
            }
        }
        let mut visited = HashMap::new();
        let mut current = (0, 0);
        let mut current_distance = 0;
        unvisited_heap.push(Reverse(Node {
            coords: current,
            weight: current_distance,
        }));
        unvisited.insert(current, current_distance);

        'outer: loop {
            for ((x, y), distance) in distances.get(&current).unwrap() {
                let neighbor = (*x, *y);
                if visited.contains_key(&neighbor.clone()) {
                    continue;
                }
                let new_distance = current_distance + distance;
                let unvisited_neighbor = *unvisited.get(&neighbor).unwrap_or(&i32::MAX);
                if unvisited_neighbor == i32::MAX || unvisited_neighbor > new_distance {
                    unvisited.insert(neighbor, new_distance);
                    unvisited_heap.push(Reverse(Node {
                        coords: neighbor,
                        weight: new_distance,
                    }));
                }
            }
            visited.insert(current, current_distance);
            visited_size += 1;
            unvisited.remove(&current);
            if visited_size == total_size || current == (x_size - 1, y_size - 1) {
                break 'outer;
            }
            let next;
            loop {
                let possible_next = unvisited_heap.pop().unwrap();
                if unvisited.contains_key(&possible_next.0.coords) {
                    next = (possible_next.0.coords, possible_next.0.weight);
                    break;
                }
            }
            current = next.0;
            current_distance = next.1;
        }

        current_distance
    }
}

fn get_map_from_input(input: &str) -> (HashMap<Coord, i32>, usize, usize) {
    let mut x_size = 0;
    let mut y_size = 0;
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        y_size = y_size.max(y);
        for (x, c) in line.chars().enumerate() {
            x_size = x_size.max(x);
            map.insert((x, y), c.to_string().parse::<i32>().unwrap());
        }
    }
    x_size += 1;
    y_size += 1;
    (map, x_size, y_size)
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let (map, x_size, y_size) = get_map_from_input(input);

        let map = Mapping::from_map(&map, x_size, y_size);
        let answer = map.solve();

        debug_assert_eq!(answer, 373);
        format!("{}", answer) // 373/~10000μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let (mut map, x_size, y_size) = get_map_from_input(input);

        for n_x in 0..=4 {
            for n_y in 0..=4 {
                if n_x == 0 && n_y == 0 {
                    continue;
                }
                for x in 0..x_size {
                    for y in 0..y_size {
                        let curr = map.get(&(x, y)).unwrap();
                        let curr = curr + n_x + n_y;
                        let curr = if curr > 9 { (curr % 10) + 1 } else { curr };
                        let new_x = x + (x_size * n_x as usize);
                        let new_y = y + (y_size * n_y as usize);
                        map.insert((new_x, new_y), curr);
                    }
                }
            }
        }
        let x_size = x_size * 5;
        let y_size = y_size * 5;

        let map = Mapping::from_map(&map, x_size, y_size);

        let answer = map.solve();
        debug_assert_eq!(answer, 2868);
        format!("{}", answer) // 2868/~330000μs
    }
}
