use crate::common::*;
use ndarray::Array2;

type Lookup = [bool; 512];

fn parse(lines: Lines) -> Result<(Lookup, Array2<bool>)> {
    ensure!(lines.len() > 2);

    let mut lookup = [false; 512];
    ensure!(lines[0].len() == lookup.len());

    for (i, c) in enumerate(lines[0].chars()) {
        lookup[i] = c == '#';
    }

    let height = lines.len() - 2;
    let width = lines[2].len();
    let mut img = Array2::from_elem((height, width), false);

    for (i, line) in enumerate(&lines[2..]) {
        ensure!(line.len() == width);

        for (j, c) in enumerate(line.chars()) {
            img[[i, j]] = c == '#';
        }
    }

    Ok((lookup, img))
}

fn simulate(input: &Array2<bool>, lookup: &Lookup, default: bool) -> Array2<bool> {
    let (n, m) = input.dim();

    Array2::from_shape_fn((n + 2, m + 2), |(i, j)| {
        let mut index = 0;

        for di in [2, 1, 0] {
            for dj in [2, 1, 0] {
                let flag = if i >= di && j >= dj {
                    input.get((i - di, j - dj)).copied().unwrap_or(default)
                } else {
                    default
                };

                index = (index * 2) + (flag as usize);
            }
        }

        lookup[index]
    })
}

fn count_after(img: &Array2<bool>, lookup: &Lookup, iters: usize) -> usize {
    let mut img = img.clone();

    for iter in 0..iters {
        img = simulate(&img, lookup, lookup[0] && (iter % 2 == 1));
    }

    img.iter().filter(|&&b| b).count()
}

pub(crate) fn run(lines: Lines) -> Result {
    let (lookup, img) = parse(lines)?;

    println!("part A: {:?}", count_after(&img, &lookup, 2));
    println!("part B: {:?}", count_after(&img, &lookup, 50));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> (Lookup, Array2<bool>) {
        let lines = [
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.\
            ###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.\
            ##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.##\
            ##.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.\
            #.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#..\
            ....#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#\
            ..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        let (lookup, img) = input();
        assert_eq!(count_after(&img, &lookup, 0), 10);
        assert_eq!(count_after(&img, &lookup, 1), 24);
        assert_eq!(count_after(&img, &lookup, 2), 35);
    }

    #[test]
    fn test_b() {
        let (lookup, img) = input();
        assert_eq!(count_after(&img, &lookup, 50), 3351);
    }
}
