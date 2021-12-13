use crate::common::*;
use ndarray::Array2;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Recap, Copy, Clone)]
#[recap(regex = r#"fold along (?P<axis>[xy])=(?P<pos>\d+)"#)]
struct Instruction {
    axis: char,
    pos: usize,
}

fn parse_grid(lines: Lines) -> Result<Array2<bool>> {
    let mut coords = vec![];
    let mut size = 0;

    for line in lines {
        let (x, y) = line
            .split_once(',')
            .ok_or_else(|| anyhow!("invalid line: {:?}", line))?;
        let (x, y) = (x.parse()?, y.parse()?);

        size = size.max(x + 1).max(y + 1);
        coords.push((x, y));
    }

    let mut grid = Array2::from_elem((size, size), false);

    for (x, y) in coords {
        grid[[x, y]] = true;
    }

    Ok(grid)
}

fn parse_instrs(lines: Lines) -> Result<Vec<Instruction>> {
    lines.iter().map(|line| Ok(line.parse()?)).collect()
}

fn fold_x(grid: &mut Array2<bool>, fold: usize) {
    let (w, h) = grid.dim();

    for x in fold..w {
        for y in 0..h {
            if grid[[x, y]] {
                grid[[x, y]] = false;

                let x2 = 2 * fold - x;
                grid[[x2, y]] = true;
            }
        }
    }
}

fn fold_y(grid: &mut Array2<bool>, fold: usize) {
    let (w, h) = grid.dim();

    for x in 0..w {
        for y in fold..h {
            if grid[[x, y]] {
                grid[[x, y]] = false;

                let y2 = 2 * fold - y;
                grid[[x, y2]] = true;
            }
        }
    }
}

fn fold(grid: &mut Array2<bool>, instrs: &[Instruction]) {
    for instr in instrs {
        match instr.axis {
            'x' => fold_x(grid, instr.pos),
            'y' => fold_y(grid, instr.pos),
            other => panic!("invalid axis: {:?}", other),
        }
    }
}

fn count_after_one_fold(grid: &Array2<bool>, instr: Instruction) -> usize {
    let mut grid = grid.clone();
    fold(&mut grid, &[instr]);
    sum(map(grid, |x| x as usize))
}

pub(crate) fn run(lines: Lines) -> Result {
    let mid = lines.iter().position(|x| x.is_empty()).unwrap();
    let mut grid = parse_grid(&lines[..mid])?;
    let instrs = parse_instrs(&lines[mid + 1..])?;

    println!("part A: {:?}", count_after_one_fold(&grid, instrs[0]));

    println!("part B:");
    fold(&mut grid, &instrs);

    for y in 0..10 {
        for x in 0..50 {
            print!("{}", [' ', 'x'][grid[[x, y]] as usize]);
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> (Array2<bool>, Vec<Instruction>) {
        let lines = [
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];

        let grid = parse_grid(&lines[..18]).unwrap();
        let instrs = parse_instrs(&lines[19..]).unwrap();

        (grid, instrs)
    }

    #[test]
    fn test_a() {
        let (grid, instrs) = input();
        assert_eq!(count_after_one_fold(&grid, instrs[0]), 17);
    }

    #[test]
    fn test_b() {
        //
    }
}
