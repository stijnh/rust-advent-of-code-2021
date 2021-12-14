use crate::common::*;
use defaultmap::DefaultHashMap;
use recap::Recap;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Recap, Copy, Clone)]
#[recap(regex = r#"(?P<lhs>[A-Z])(?P<rhs>[A-Z]) -> (?P<output>[A-Z])"#)]
struct Rule {
    lhs: char,
    rhs: char,
    output: char,
}

fn parse(lines: Lines) -> Result<Vec<Rule>> {
    lines.iter().map(|line| Ok(Rule::from_str(line)?)).collect()
}

fn count_most_minus_least(input: &str, rules: &[Rule], steps: usize) -> usize {
    let mut current = DefaultHashMap::<_, usize>::new(0);

    let last = input.chars().next_back().unwrap();
    current[(last, '\0')] += 1; // Add dummy to ensure final character is counted

    for (a, b) in input.chars().tuple_windows() {
        current[(a, b)] += 1;
    }

    for _ in 0..steps {
        let mut next = DefaultHashMap::<_, usize>::new(0);

        for ((a, b), count) in current.drain() {
            if let Some(rule) = find(rules, |r| [r.lhs, r.rhs] == [a, b]) {
                let x = rule.output;
                next[(a, x)] += count;
                next[(x, b)] += count;
            } else {
                next[(a, b)] += count;
            }
        }

        current = next;
    }

    let mut counts = DefaultHashMap::new(0);

    for ((a, _), c) in current.drain() {
        counts[a] += c;
    }

    let counts = counts.values().copied().sorted().collect_vec();
    counts.last().unwrap() - counts.first().unwrap()
}

pub(crate) fn run(lines: Lines) -> Result {
    let input = lines[0];
    let rules = parse(&lines[2..])?;

    println!("part A: {:?}", count_most_minus_least(input, &rules, 10));
    println!("part B: {:?}", count_most_minus_least(input, &rules, 40));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> (String, Vec<Rule>) {
        let input = "NNCB".to_string();
        let lines = [
            "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C",
            "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C",
        ];

        (input, parse(&lines).unwrap())
    }

    #[test]
    fn test_a() {
        let (input, rules) = input();
        assert_eq!(count_most_minus_least(&input, &rules, 10), 1588);
    }

    #[test]
    fn test_b() {
        let (input, rules) = input();
        assert_eq!(count_most_minus_least(&input, &rules, 40), 2188189693529);
    }
}
