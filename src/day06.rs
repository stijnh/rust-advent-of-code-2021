use crate::common::*;

const N: usize = 16;
type Population = Box<[u128; N]>;

fn parse_population(line: &str) -> Result<Population> {
    let mut pop = Population::default();

    for num in line.split(",") {
        let n: usize = num.parse()?;
        pop[n] += 1;
    }

    Ok(pop)
}

fn simulate_day(input: Population) -> Population {
    let mut output = Population::default();

    output[6] += input[0];
    output[8] += input[0];

    for i in 1..N {
        output[i - 1] += input[i];
    }

    output
}

fn population_after_days(mut fish: Population, days: usize) -> u128 {
    for _ in 0..days {
        fish = simulate_day(fish);
    }

    fish.iter().sum()
}

pub(crate) fn run(lines: Lines) -> Result {
    let initial = parse_population(&lines[0])?;

    let total = population_after_days(initial.clone(), 80);
    println!("part A: {:?}", total);

    let total = population_after_days(initial.clone(), 256);
    println!("part B: {:?}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Population {
        parse_population("3,4,3,1,2").unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(population_after_days(input(), 80), 5934);
    }

    #[test]
    fn test_b() {
        assert_eq!(population_after_days(input(), 256), 26984457539);
    }
}
