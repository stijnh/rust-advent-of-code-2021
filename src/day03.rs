use crate::common::*;

type Number = i32;

fn parse(input: Lines) -> Result<Vec<Number>> {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|line| Ok(i32::from_str_radix(line, 2)?))
        .collect()
}

fn solve_a(input: &[Number], n: i32) -> (Number, Number) {
    let mut gamma = 0;

    for p in 0..n {
        let mask = 1 << p;
        let ones = input
            .iter()
            .filter(|&&number| number & mask == mask)
            .count();

        if 2 * ones > input.len() {
            gamma |= mask;
        }
    }

    let epsilon = (!gamma) & ((1 << n) - 1);

    (gamma, epsilon)
}

#[derive(PartialEq, Eq)]
enum Rating {
    Oxygen,
    Co2,
}

fn solve_b(input: &[Number], item: Rating, n: i32) -> Number {
    let mut input = input.to_vec();

    for p in (0..n).rev() {
        let mask = 1 << p;
        let ones = input
            .iter()
            .filter(|&&number| number & mask == mask)
            .count();

        let zeros = input.len() - ones;
        let bit = if (ones >= zeros) ^ (item == Rating::Oxygen) {
            mask
        } else {
            0
        };

        input.retain(|&number| number & mask == bit);

        if input.len() == 1 {
            return input[0];
        }
    }

    panic!("number not found!");
}

pub(crate) fn run(lines: Lines) -> Result {
    let lines = parse(lines)?;

    let (gamma, epsilon) = solve_a(&lines, 12);
    println!("part A: {}", gamma * epsilon);

    let oxy = solve_b(&lines, Rating::Oxygen, 12);
    let co2 = solve_b(&lines, Rating::Co2, 12);
    println!("part A: {}", oxy * co2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Number> {
        let lines = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        let (gamma, epsilon) = solve_a(&input(), 5);

        assert_eq!((gamma, epsilon), (22, 9));
    }

    #[test]
    fn test_b() {
        let oxy = solve_b(&input(), Rating::Oxygen, 5);
        let co2 = solve_b(&input(), Rating::Co2, 5);

        assert_eq!((oxy, co2), (10, 23));
    }
}
