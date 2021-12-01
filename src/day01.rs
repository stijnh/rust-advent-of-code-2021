use crate::common::*;

fn solve_a(numbers: &[u32]) -> usize {
    numbers.windows(2).filter(|w| w[0] < w[1]).count()
}

fn solve_b(numbers: &[u32]) -> usize {
    numbers.windows(4).filter(|w| w[0] < w[3]).count()
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = lines
        .iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("error while parsing input"))?;

    let n = solve_a(&numbers);
    println!("part A: {}", n);

    let n = solve_b(&numbers);
    println!("part B: {}", n);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let numbers = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(solve_a(&numbers), 7);
    }

    #[test]
    fn test_b() {
        let numbers = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(solve_b(&numbers), 5);
    }
}
