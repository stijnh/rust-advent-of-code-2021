use crate::common::*;

fn solve<F: Fn(i32) -> i32>(pos: &[i32], fuel: F) -> i32 {
    let (&min, &max) = pos.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|p| pos.iter().map(|x| fuel((p - x).abs())).sum())
        .min()
        .unwrap()
}

fn solve_a(pos: &[i32]) -> i32 {
    solve(pos, |dist| dist)
}

fn solve_b(pos: &[i32]) -> i32 {
    solve(pos, |dist| dist * (dist + 1) / 2)
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = parse_list(lines[0], ',')?;

    println!("part A: {}", solve_a(&numbers));
    println!("part B: {}", solve_b(&numbers));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<i32> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn test_a() {
        assert_eq!(solve_a(&input()), 37)
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(&input()), 168)
    }
}
