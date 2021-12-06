use crate::common::*;

#[derive(Copy, Clone)]
enum Action {
    Forward(i32),
    Down(i32),
    Up(i32),
}

use Action::*;

fn parse(lines: Lines) -> Result<Vec<Action>> {
    lines
        .iter()
        .map(|line| {
            let (command, length) = line.trim().split_once(" ").context("invalid line")?;

            let n = length.parse::<i32>()?;
            let action = match command {
                "forward" => Forward(n),
                "down" => Down(n),
                "up" => Up(n),
                other => bail!("invalid action: {:?}", other),
            };

            Ok(action)
        })
        .collect()
}

fn solve_a(actions: &[Action]) -> i32 {
    let mut forward = 0;
    let mut depth = 0;

    for &action in actions {
        match action {
            Forward(v) => forward += v,
            Down(v) => depth += v,
            Up(v) => depth -= v,
        }
    }

    forward * depth
}

fn solve_b(actions: &[Action]) -> i32 {
    let mut forward = 0;
    let mut aim = 0;
    let mut depth = 0;

    for &action in actions {
        match action {
            Forward(v) => {
                forward += v;
                depth += v * aim;
            }
            Down(v) => aim += v,
            Up(v) => aim -= v,
        }
    }

    forward * depth
}

pub(crate) fn run(lines: Lines) -> Result {
    let actions = parse(lines)?;

    let n = solve_a(&actions);
    println!("part A: {}", n);

    let n = solve_b(&actions);
    println!("part B: {}", n);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Action> {
        let lines = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(solve_a(&input()), 150);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(&input()), 900);
    }
}
