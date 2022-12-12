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
    // let input = input.to_owned();
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
    // let computer: &mut Computer = computer;
    // let res = computer.run_command();
    dbg!(res);
    todo!();
}

// #[aoc(day02, part2)]
// fn part_2(_input: &str) -> u32 {
//     todo!();
// }

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

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&input), 0);
    // }
}
