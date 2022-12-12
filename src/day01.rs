#[aoc(day1, part1)]
fn part_1(input: &str) -> i32 {
    input.lines().map(solve).sum()
}

fn solve(input: &str) -> i32 {
    solve_int(input.parse::<i32>().unwrap())
}

fn solve_int(input: i32) -> i32 {
    (input as i32 / 3) - 2
}

fn solve_rec(fuel: i32) -> i32 {
    let fuel = solve_int(fuel);
    if fuel <= 0 {
        0
    } else {
        fuel + solve_rec(fuel)
    }
}

#[aoc(day1, part2)]
fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| solve_rec(line.parse().unwrap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "12";
    const INPUT_2: &str = "14";
    const INPUT_3: &str = "1969";
    const INPUT_4: &str = "100756";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(INPUT_1), 2);
        assert_eq!(solve(INPUT_2), 2);
        assert_eq!(solve(INPUT_3), 654);
        assert_eq!(solve(INPUT_4), 33583);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_rec(14), 2);
        assert_eq!(solve_rec(1969), 966);
        assert_eq!(solve_rec(100756), 50346);
    }
}
