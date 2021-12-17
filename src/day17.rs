use crate::common::*;
use recap::Recap;
use serde::Deserialize;

type Num = i32;

#[derive(Debug, Deserialize, PartialEq, Recap, Copy, Clone)]
#[recap(
    regex = r#"target area: x=(?P<x0>[0-9]+)..(?P<x1>[0-9]+), y=(?P<y0>-[0-9]+)..(?P<y1>-[0-9]+)"#
)]
struct Target {
    x0: Num,
    y0: Num,
    x1: Num,
    y1: Num,
}

fn simulate(mut vx: Num, mut vy: Num, target: Target) -> Option<Num> {
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;

    loop {
        x += vx;
        y += vy;
        max_y = max_y.max(y);

        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }

        vy -= 1;

        let in_x = (target.x0..=target.x1).contains(&x);
        let in_y = (target.y0..=target.y1).contains(&y);

        if in_x && in_y {
            return Some(max_y);
        }

        if !in_x && vx == 0 {
            return None;
        }

        if !in_y && vy < 0 && y < target.y0 {
            return None;
        }
    }
}

fn highest_position(target: Target) -> Num {
    let mut best = 0;

    for vx in 0..=100 {
        for vy in 0..=100 {
            if let Some(max_y) = simulate(vx, vy, target) {
                if max_y > best {
                    best = max_y
                }
            }
        }
    }

    best
}

fn number_velocities(target: Target) -> Num {
    let mut total = 0;

    for vx in 0..=350 {
        for vy in -100..=100 {
            total += simulate(vx, vy, target).is_some() as Num;
        }
    }

    total
}

pub(crate) fn run(lines: Lines) -> Result {
    let target = lines[0].parse()?;

    println!("part A: {:?}", highest_position(target));
    println!("part B: {:?}", number_velocities(target));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Target {
        "target area: x=20..30, y=-10..-5".parse().unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(highest_position(input()), 45);
    }

    #[test]
    fn test_b() {
        assert_eq!(number_velocities(input()), 112);
    }
}
