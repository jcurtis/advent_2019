use itertools::Itertools;

type Input = Vec<Vec<(i32, i32)>>;

#[aoc_generator(day3)]
fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let wire = vec![(0, 0)];
            // println!("For wire {}", line);
            let res = line.split(',').fold(wire, |acc, ins| {
                let direction = ins.chars().next().unwrap();
                let distance = &ins[1..];
                let distance = distance.parse::<i32>().unwrap();
                let &(x, y) = acc.last().unwrap();

                let line = match direction {
                    'R' => ((x + 1)..(x + distance + 1)).map(|x| (x, y)).collect_vec(),
                    'L' => ((x - distance)..x).map(|x| (x, y)).rev().collect_vec(),
                    'U' => ((y + 1)..(y + distance + 1)).map(|y| (x, y)).collect_vec(),
                    'D' => ((y - distance)..y).map(|y| (x, y)).rev().collect_vec(),
                    _ => unreachable!(),
                };
                // println!(
                //     "From ({x}, {y}) with {direction}{distance} generate: {:?}",
                //     line
                // );

                vec![acc, line].concat()
            });
            res
        })
        .collect_vec()
}

#[aoc(day3, part1)]
fn part_1(wires: &Input) -> i32 {
    let wire1 = &wires[0];
    let wire2 = &wires[1];

    wire1
        .iter()
        .filter(|&pos| pos != &(0, 0) && wire2.contains(pos))
        .map(|&(x, y)| x.abs() + y.abs())
        .sorted()
        .next()
        .unwrap()
}

#[aoc(day3, part2)]
fn part_2(wires: &Input) -> usize {
    let wire1 = &wires[0];
    let wire2 = &wires[1];

    wire1
        .iter()
        .filter(|&pos| pos != &(0, 0) && wire2.contains(pos))
        .map(|intersection| {
            wire1.iter().position(|pos| pos == intersection).unwrap()
                + wire2.iter().position(|pos| pos == intersection).unwrap()
        })
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_0: &str = "R8,U5,L5,D3
U7,R6,D4,L4";
    const SAMPLE_1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    const SAMPLE_2: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_part_1() {
        let input = generator(SAMPLE_0);
        assert_eq!(part_1(&input), 6);

        let input = generator(SAMPLE_1);
        assert_eq!(part_1(&input), 159);

        let input = generator(SAMPLE_2);
        assert_eq!(part_1(&input), 135);
    }

    #[test]
    fn test_part_2() {
        let input = generator(SAMPLE_0);
        assert_eq!(part_2(&input), 30);

        let input = generator(SAMPLE_1);
        assert_eq!(part_2(&input), 610);

        let input = generator(SAMPLE_2);
        assert_eq!(part_2(&input), 410);
    }
}
