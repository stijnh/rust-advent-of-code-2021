use crate::common::*;
use ndarray::{Array2, ArrayView2};

fn parse(lines: Lines) -> Result<Array2<char>> {
    let height = lines.len();
    let width = lines[0].len();
    let mut map = Array2::from_elem((height, width), '.');

    for (i, line) in enumerate(lines) {
        ensure!(line.len() == width, "invalid input");

        for (j, c) in enumerate(line.chars()) {
            map[[i, j]] = c;
        }
    }

    Ok(map)
}

fn evolve(input: ArrayView2<char>) -> Array2<char> {
    let (height, width) = input.dim();
    let mut output = input.to_owned();

    for i in 0..height {
        for j in 0..width {
            if input[[i, j]] == '>' && input[[i, (j + 1) % width]] == '.' {
                output[[i, j]] = '.';
                output[[i, (j + 1) % width]] = '>';
            }
        }
    }

    let input = output.clone();

    for i in 0..height {
        for j in 0..width {
            if input[[i, j]] == 'v' && input[[(i + 1) % height, j]] == '.' {
                output[[i, j]] = '.';
                output[[(i + 1) % height, j]] = 'v';
            }
        }
    }

    output
}

fn evolve_forever(mut current: Array2<char>) -> usize {
    for steps in 1.. {
        let prev = current;
        current = evolve(prev.view());

        if current == prev {
            return steps;
        }
    }

    unreachable!()
}

pub(crate) fn run(lines: Lines) -> Result {
    let map = parse(lines)?;
    println!("part A: {}", evolve_forever(map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let lines = [
            "v...>>.vv>",
            ".vv>>.vv..",
            ">>.>v>...v",
            ">>v>>.>.v.",
            "v>v.vv.v..",
            ">.>>..v...",
            ".vv..>.>v.",
            "v.v..>>v.v",
            "....v..v.>",
        ];

        let map = parse(&lines).unwrap();
        assert_eq!(evolve_forever(map), 58);
    }

    #[test]
    fn test_b() {
        //
    }
}
