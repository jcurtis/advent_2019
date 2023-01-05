use std::ops::RangeInclusive;

use itertools::Itertools;

type Input = RangeInclusive<u32>;

#[aoc_generator(day4)]
fn generator(input: &str) -> Input {
    let range: Vec<u32> = input.split('-').map(|num| num.parse().unwrap()).collect();
    range[0]..=range[1]
}

#[aoc(day4, part1)]
fn part_1(range: &Input) -> usize {
    range.to_owned().filter(|&num| test_password_1(num)).count()
}

fn test_password_1(input: u32) -> bool {
    let input = input.to_string().bytes().collect_vec();
    if input.len() != 6 {
        return false;
    }

    let mut adjacent = false;

    let never_decrease = !input.iter().enumerate().skip(1).any(|(i, c)| {
        let prev = input[i - 1];
        let c = *c;

        if prev == c {
            adjacent = true;
        }
        prev > c
    });

    adjacent && never_decrease
}

#[aoc(day4, part2)]
fn part_2(range: &Input) -> usize {
    range.to_owned().filter(|&num| test_password_2(num)).count()
}

fn test_password_2(input: u32) -> bool {
    let input = input.to_string().bytes().collect_vec();
    if input.len() != 6 {
        return false;
    }

    let mut adjacent = false;

    let never_decrease = !input.iter().enumerate().skip(1).any(|(i, c)| {
        let c = *c;
        let prev = input[i - 1];

        if prev == c {
            if (i < 5 && input[i + 1] == c) || (i > 1 && input[i - 2] == c) {
                // todo
            } else {
                adjacent = true;
            }
        }
        prev > c
    });

    adjacent && never_decrease
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = generator("111111-111112");
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_test_password_1() {
        assert!(test_password_1(111111));
        assert!(!test_password_1(223450));
        assert!(!test_password_1(123789));
    }

    #[test]
    fn test_part_2() {
        let input = generator("111111-111112");
        assert_eq!(part_2(&input), 0);
    }

    #[test]
    fn test_test_password_2() {
        assert!(test_password_2(112233));
        assert!(!test_password_2(123444));
        assert!(test_password_2(111122));
    }
}
