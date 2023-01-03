use itertools::Itertools;

use crate::computer::Computer;

#[aoc_generator(day02)]
fn generator(input: &str) -> Computer {
    Computer::from(input)
}

#[aoc(day02, part1)]
fn part_1(input: &Computer) -> u32 {
    let mut memory = input.memory.to_owned();
    memory[1] = 12;
    memory[2] = 2;
    solve(input)
}

fn solve(input: &Computer) -> u32 {
    let mut input = input.memory.to_owned();
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            99 => {
                // println!("99: break");
                break;
            }
            1 => {
                let a = input[input[i + 1] as usize];
                let b = input[input[i + 2] as usize];
                let to = input[i + 3] as usize;
                input[to] = a + b;
                // println!("1: {a} + {b} into {to}");
                i += 4;
            }
            2 => {
                let a = input[input[i + 1] as usize];
                let b = input[input[i + 2] as usize];
                let to = input[i + 3] as usize;
                input[to] = a * b;
                // println!("2: {a} * {b} into {to}");
                i += 4;
            }
            _ => todo!(),
        }
    }
    // println!("end: {:?}", &input);
    input[0]
}

#[aoc(day02, part1, Computer)]
fn part_1_computer(computer: &Computer) -> u32 {
    if let Ok(res) = solve_computer(computer, Some((12, 2))) {
        res
    } else {
        unreachable!()
    }
}

fn solve_computer(computer: &Computer, parameters: Option<(u32, u32)>) -> Result<u32, String> {
    let mut computer = computer.clone();
    computer.run_program(parameters)
}

const GOAL: u32 = 19690720;
#[aoc(day02, part2)]
fn part_2(computer: &Computer) -> u32 {
    let mut flip = false;
    let pair = (0..=99).combinations(2).find(|parameters| {
        let noun = parameters[0];
        let verb = parameters[1];
        if let Ok(res) = solve_computer(computer, Some((noun, verb))) {
            if res == GOAL {
                return true;
            }
        }
        if let Ok(res) = solve_computer(computer, Some((verb, noun))) {
            if res == GOAL {
                flip = true;
                return true;
            }
        }
        false
    });

    if let Some(pair) = pair {
        let mut noun = pair[0];
        let mut verb = pair[1];
        if flip {
            noun = pair[1];
            verb = pair[0];
        }
        (100 * noun) + verb
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
    const SAMPLE_2: &str = "1,0,0,0,99";
    const SAMPLE_3: &str = "2,3,0,3,99";
    const SAMPLE_4: &str = "1,1,1,4,99,5,6,0,99";

    #[test]
    fn test_part_1() {
        let input = generator(SAMPLE_1);
        assert_eq!(part_1(&input), 3500);

        let input = generator(SAMPLE_2);
        assert_eq!(part_1(&input), 2);

        let input = generator(SAMPLE_3);
        assert_eq!(part_1(&input), 2);

        let input = generator(SAMPLE_4);
        assert_eq!(part_1(&input), 30);
    }

    #[test]
    fn test_solve_computer() {
        let input = generator(SAMPLE_1);
        assert_eq!(solve_computer(&input, None), Ok(3500));

        let input = generator(SAMPLE_2);
        assert_eq!(solve_computer(&input, None), Ok(2));

        let input = generator(SAMPLE_3);
        assert_eq!(solve_computer(&input, None), Ok(2));

        let input = generator(SAMPLE_4);
        assert_eq!(solve_computer(&input, None), Ok(30));
    }

    // #[test]
    // this example doesn't make sense
    fn _test_part_2() {
        let input = generator(SAMPLE_1);
        assert_eq!(solve_computer(&input, Some((12, 2))), Ok(19690720));
        assert_eq!(part_2(&input), 1202);
    }
}
