use crate::common::*;
use ndarray::{Array2, ArrayView2};

type Num = i32;

fn neighbors(map: ArrayView2<Num>, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let (n, m) = map.dim();

    [[0, 1], [0, -1], [1, 0], [-1, 0]]
        .iter()
        .map(move |&[dx, dy]| (x as isize + dx, y as isize + dy))
        .filter_map(|(x, y)| Some((x.try_into().ok()?, y.try_into().ok()?)))
        .filter(move |&(x, y)| x < n && y < m)
}

fn parse(lines: Lines) -> Result<Array2<Num>> {
    let n = lines.len();
    let m = lines[0].len();
    let mut grid = Array2::from_elem((n, m), 0);

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            grid[[i, j]] = c.to_string().parse()?;
        }
    }

    Ok(grid)
}

fn solve_a(map: ArrayView2<Num>) -> Num {
    map.indexed_iter()
        .map(|((i, j), &value)| {
            let is_low = all(neighbors(map.view(), i, j), |neighbor| {
                map[neighbor] > value
            });

            is_low as Num * (1 + value)
        })
        .sum()
}

fn solve_b(map: ArrayView2<Num>) -> usize {
    let mut next_label = 0;
    let mut labels = Array2::<Option<Num>>::from_elem(map.dim(), None);

    for ((i, j), &value) in map.indexed_iter() {
        if value != 9 {
            labels[[i, j]] = Some(next_label);
            next_label += 1;
        }
    }

    loop {
        let mut changes = 0;

        for ((i, j), _) in map.indexed_iter() {
            for neighbor in neighbors(map.view(), i, j) {
                if let Some(my_label) = labels[[i, j]] {
                    if let Some(his_label) = labels[neighbor] {
                        if my_label > his_label {
                            labels[[i, j]] = Some(his_label);
                            changes += 1;
                        }
                    }
                }
            }
        }

        if changes == 0 {
            break;
        }
    }

    let mut counts = HashMap::default();
    for label in flatten(labels) {
        *counts.entry(label).or_default() += 1;
    }

    counts.values().sorted().rev().take(3).product()
}

pub(crate) fn run(lines: Lines) -> Result {
    let map = parse(lines)?;

    println!("part A: {:?}", solve_a(map.view()));
    println!("part B: {:?}", solve_b(map.view()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Array2<Num> {
        let lines = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(solve_a(input().view()), 15);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(input().view()), 1134);
    }
}
