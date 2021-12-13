use crate::common::*;
use ndarray::Array2;
use std::collections::HashSet;

const N: usize = 10;

fn parse(lines: Lines) -> Result<Array2<i32>> {
    ensure!(lines.len() == N, "invalid input");
    let mut grid = Array2::zeros((N, N));

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            let x = c.to_string().parse()?;
            ensure!(x >= 0 && x <= 9 && j < N, "invalid character: {:?}", x);

            grid[[i, j]] = x;
        }
    }

    Ok(grid)
}

fn neighbors(i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, 1],
        [0, -1],
        [1, 1],
        [1, 0],
        [1, -1],
    ]
    .iter()
    .map(move |&[di, dj]| (i as isize + di, j as isize + dj))
    .filter_map(|(x, y)| Some((x.try_into().ok()?, y.try_into().ok()?)))
    .filter(move |&(x, y)| x < N && y < N)
}

fn step(grid: &mut Array2<i32>) -> usize {
    assert_eq!(grid.shape(), &[N, N]);
    let mut active = Vec::new();
    let mut visited = HashSet::new();

    // First, the energy level of each octopus increases by 1.
    for i in 0..N {
        for j in 0..N {
            grid[[i, j]] += 1;

            // any octopus with an energy level greater than 9 flashes.
            if grid[[i, j]] > 9 {
                active.push((i, j));
                visited.insert((i, j));
            }
        }
    }

    while let Some((i, j)) = active.pop() {
        for neighbor in neighbors(i, j) {
            if visited.contains(&neighbor) {
                continue;
            }

            // increases the energy level of all adjacent octopuses by 1,
            // including octopuses that are diagonally adjacent
            grid[neighbor] += 1;

            // If this causes an octopus to have an energy level greater than 9, it also flashes.
            if grid[neighbor] > 9 {
                active.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    for &index in &visited {
        grid[index] = 0;
    }

    visited.len()
}

fn count_flashes(input: &Array2<i32>, steps: usize) -> usize {
    let mut total = 0;
    let mut grid = input.clone();

    for _ in 0..steps {
        total += step(&mut grid);
    }

    total
}

fn first_simulate_flash(input: &Array2<i32>) -> usize {
    let mut steps = 0;
    let mut grid = input.clone();

    loop {
        steps += 1;

        if step(&mut grid) == N * N {
            return steps;
        }
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let grid = parse(lines)?;

    println!("part A: {:?}", count_flashes(&grid, 100));
    println!("part B: {:?}", first_simulate_flash(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Array2<i32> {
        let lines = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(count_flashes(&input(), 1), 0);
        assert_eq!(count_flashes(&input(), 2), 35);
        assert_eq!(count_flashes(&input(), 100), 1656);
    }

    #[test]
    fn test_b() {
        assert_eq!(first_simulate_flash(&input()), 195);
    }
}
