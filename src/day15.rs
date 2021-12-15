use crate::common::*;
use binary_heap_plus::BinaryHeap;
use defaultmap::DefaultHashMap;
use ndarray::{Array2, ArrayView2};
use std::cmp::Reverse;

fn parse(lines: Lines) -> Result<Array2<u32>> {
    let h = lines.len();
    let w = lines[0].len();
    let mut map = Array2::from_elem((w, h), 0);

    for (y, line) in enumerate(lines) {
        ensure!(line.len() == w, "invalid input");

        for (x, c) in enumerate(line.chars()) {
            map[[x, y]] = c
                .to_digit(10)
                .ok_or_else(|| anyhow!("invalid digit: {:?}", c))?;
        }
    }

    Ok(map)
}

fn grow_map(input: ArrayView2<u32>) -> Array2<u32> {
    const FACTOR: usize = 5;
    let (n, m) = input.dim();
    let mut output = Array2::from_elem((n * FACTOR, m * FACTOR), 0);

    for a in 0..FACTOR {
        for b in 0..FACTOR {
            for x in 0..n {
                for y in 0..m {
                    output[[a * n + x, b * m + y]] = (input[[x, y]] + (a + b) as u32 - 1) % 9 + 1;
                }
            }
        }
    }

    output
}

fn neighbors<T>(map: ArrayView2<T>, [x, y]: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    let (n, m) = map.dim();

    [[0, 1], [0, -1], [1, 0], [-1, 0]]
        .iter()
        .map(move |&[dx, dy]| [x as isize + dx, y as isize + dy])
        .filter_map(|[x, y]| Some([x.try_into().ok()?, y.try_into().ok()?]))
        .filter(move |&[x, y]| x < n && y < m)
}

fn lowest_risk(map: ArrayView2<u32>) -> u32 {
    let mut risk = DefaultHashMap::new(u32::MAX);
    let mut queue = BinaryHeap::new_by_key(|&(_, risk)| Reverse(risk));

    queue.push(([0, 0], 0));

    while let Some((pos, current)) = queue.pop() {
        if current >= risk[pos] {
            continue;
        }

        risk[pos] = current;

        for neighbor in neighbors(map.view(), pos) {
            queue.push((neighbor, current + map[neighbor]));
        }
    }

    let (n, m) = map.dim();
    risk[[n - 1, m - 1]]
}

pub(crate) fn run(lines: Lines) -> Result {
    let map = parse(lines)?;

    println!("part A: {:?}", lowest_risk(map.view()));
    println!("part B: {:?}", lowest_risk(grow_map(map.view()).view()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Array2<u32> {
        let lines = [
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(lowest_risk(input().view()), 40);
    }

    #[test]
    fn test_b() {
        assert_eq!(lowest_risk(grow_map(input().view()).view()), 315);
    }
}
