use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::bfs;

type Label = String;
type Input = HashMap<Label, Vec<Label>>;

#[aoc_generator(day6)]
fn generator(input: &str) -> Input {
    let mut map: Input = HashMap::new();
    input.lines().for_each(|line| {
        let split = line.split(')').collect_vec();
        assert_eq!(split.len(), 2);
        let entry = map.get_mut(split[0]);
        match entry {
            Some(entry) => {
                entry.push(split[1].to_string());
            }
            None => {
                map.insert(split[0].to_string(), vec![split[1].to_string()]);
            }
        };

        if !map.contains_key(split[1]) {
            map.insert(split[1].to_string(), vec![]);
        }
    });
    map
}

#[aoc(day6, part1)]
fn part_1(input: &Input) -> usize {
    input
        .keys()
        .map(|key| {
            // from COM to key
            let start = "COM".to_string();
            if let Some(path) = bfs(
                &start,
                |from| input.get(from).unwrap().clone(),
                |goal| goal == key,
            ) {
                // println!("Path from COM to {key}: {path:?}");
                path.len() - 1
            } else {
                // eprintln!("No path found for {key}");
                0
            }
        })
        .sum()
}

fn root_of(input: &Input, key: String) -> Option<&String> {
    if let Some(found) = input.iter().find(|&(_, entry)| entry.contains(&key)) {
        Some(found.0)
    } else {
        None
    }
}

#[aoc(day6, part2)]
fn part_2(input: &Input) -> usize {
    let you = "YOU".to_string();
    let san = "SAN".to_string();

    let start = root_of(&input, you).unwrap();
    let goal = root_of(&input, san).unwrap();

    bfs(
        start,
        |key| {
            let mut successors = input.get(key).unwrap().clone();
            if let Some(root) = root_of(&input, key.to_string()) {
                successors.push(root.to_string());
            }
            successors
        },
        |key| key == goal,
    )
    .unwrap()
    .len()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    #[test]
    fn test_part_1() {
        let input = generator(SAMPLE_1);
        assert_eq!(part_1(&input), 42);
    }

    const SAMPLE_2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_part_2() {
        let input = generator(SAMPLE_2);
        assert_eq!(part_2(&input), 4);
    }
}
