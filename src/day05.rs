use crate::common::*;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"(?P<x0>\d+),(?P<y0>\d+) -> (?P<x1>\d+),(?P<y1>\d+)"#)]
struct Segment {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

fn parse(lines: Lines) -> Result<Vec<Segment>> {
    lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect()
}

fn overlaps(segments: &[Segment], diagonals: bool) -> usize {
    let mut points: HashMap<(i32, i32), usize> = default();

    for seg in segments {
        let nx = (seg.x1 - seg.x0).abs();
        let dx = (seg.x1 - seg.x0).signum();
        let ny = (seg.y1 - seg.y0).abs();
        let dy = (seg.y1 - seg.y0).signum();

        if !(nx == 0 || ny == 0 || (nx == ny && diagonals)) {
            continue;
        }

        let n = i32::max(nx, ny);

        for i in 0..=n {
            let x = seg.x0 + dx * i;
            let y = seg.y0 + dy * i;
            *points.entry((x, y)).or_default() += 1;
        }
    }

    points.values().filter(|&&s| s > 1).count()
}

pub(crate) fn run(lines: Lines) -> Result {
    let lines = parse(lines)?;

    println!("part A: {:?}", overlaps(&lines, false));
    println!("part B: {:?}", overlaps(&lines, true));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Segment> {
        let lines = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(overlaps(&input(), false), 5);
    }

    #[test]
    fn test_b() {
        assert_eq!(overlaps(&input(), true), 12);
    }
}
