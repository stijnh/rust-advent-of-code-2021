use crate::common::*;

const N: usize = 9;

#[derive(Default, Clone)]
struct Population {
    counts: Box<[u128; N]>,
    offset: usize,
}

fn parse_population(line: &str) -> Result<Population> {
    let mut pop = Population::default();

    for n in parse_list::<usize>(line, ',')? {
        pop.counts[n] += 1;
    }

    Ok(pop)
}

fn simulate_day(mut fish: Population) -> Population {
    let offset = fish.offset;
    fish.counts[(offset + N - 2) % N] += fish.counts[offset];
    fish.offset = (offset + 1) % N;
    fish
}

fn population_after_days(mut fish: Population, days: usize) -> u128 {
    for _ in 0..days {
        fish = simulate_day(fish);
    }

    fish.counts.iter().sum()
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
