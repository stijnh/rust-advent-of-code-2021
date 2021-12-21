use crate::common::*;
use defaultmap::DefaultHashMap;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;

type Num = i64;

#[derive(Hash, Debug, Deserialize, PartialEq, Recap, Copy, Clone, Ord, PartialOrd, Eq)]
#[recap(regex = r#"(?P<x>-?[0-9]+),(?P<y>-?[0-9]+),(?P<z>-?[0-9]+)"#)]
struct Beacon {
    x: Num,
    y: Num,
    z: Num,
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: HashSet<Beacon>,
}

fn parse_scanners(lines: Lines) -> Result<Vec<Scanner>> {
    let mut lines = lines.iter().peekable();
    let mut scanners = vec![];

    while let Some(header) = lines.next() {
        if !is_match("--- scanner [0-9]+ ---", header) {
            bail!("invalid header: {:?}", header);
        }

        let mut beacons = HashSet::default();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            beacons.insert(line.parse()?);
        }

        scanners.push(Scanner { beacons });
    }

    Ok(scanners)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rotate {
    dx: [Num; 3],
    dy: [Num; 3],
    dz: [Num; 3],
}

impl Rotate {
    fn all() -> impl Iterator<Item = Self> {
        const VECTORS: [[Num; 3]; 6] = [
            [1, 0, 0],
            [-1, 0, 0],
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
        ];

        VECTORS
            .iter()
            .cartesian_product(VECTORS)
            .flat_map(|(&dx, dy)| {
                let dz = [
                    dx[1] * dy[2] - dx[2] * dy[1],
                    dx[2] * dy[0] - dx[0] * dy[2],
                    dx[0] * dy[1] - dx[1] * dy[0],
                ];

                if dz[0].abs() + dz[1].abs() + dz[2].abs() != 1 {
                    return None;
                }

                Some(Self { dx, dy, dz })
            })
    }

    fn apply(&self, b: Beacon) -> Beacon {
        let (dx, dy, dz) = (self.dx, self.dy, self.dz);

        Beacon {
            x: b.x * dx[0] + b.y * dx[1] + b.z * dx[2],
            y: b.x * dy[0] + b.y * dy[1] + b.z * dy[2],
            z: b.x * dz[0] + b.y * dz[1] + b.z * dz[2],
        }
    }

    fn inverse(&self, b: Beacon) -> Beacon {
        let (dx, dy, dz) = (self.dx, self.dy, self.dz);

        Beacon {
            x: b.x * dx[0] + b.y * dy[0] + b.z * dz[0],
            y: b.x * dx[1] + b.y * dy[1] + b.z * dz[1],
            z: b.x * dx[2] + b.y * dy[2] + b.z * dz[2],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Transform {
    rotate: Rotate,
    shift: [Num; 3],
}

impl Transform {
    fn apply(&self, b: Beacon) -> Beacon {
        let p = self.shift;
        let r = self.rotate.apply(b);

        Beacon {
            x: r.x + p[0],
            y: r.y + p[1],
            z: r.z + p[2],
        }
    }

    fn inverse(&self, b: Beacon) -> Beacon {
        let p = self.shift;
        self.rotate.inverse(Beacon {
            x: b.x - p[0],
            y: b.y - p[1],
            z: b.z - p[2],
        })
    }
}

fn num_hits(scanner: &Scanner, beacons: &HashSet<Beacon>, transform: Transform) -> Option<usize> {
    const RANGE: Num = 1000;
    let mut num = 0;

    for &before in beacons {
        let after = transform.apply(before);

        if after.x.abs() > RANGE || after.y.abs() > RANGE || after.z.abs() > RANGE {
            continue;
        }

        if !scanner.beacons.contains(&after) {
            return None;
        }

        num += 1;
    }

    Some(num)
}

fn find_orientation(scanner: &Scanner, beacons: &HashSet<Beacon>) -> Option<Transform> {
    for rotate in Rotate::all() {
        for &before in beacons {
            for &after in &scanner.beacons {
                let p = rotate.apply(before);
                let transform = Transform {
                    rotate,
                    shift: [after.x - p.x, after.y - p.y, after.z - p.z],
                };

                if let Some(num) = num_hits(scanner, beacons, transform) {
                    if num >= 12 {
                        return Some(transform);
                    }
                }
            }
        }
    }

    None
}


fn find_beacons(mut scanners: Vec<Scanner>) -> (HashSet<Beacon>, Vec<[Num; 3]>) {
    let mut beacons = HashSet::default();
    beacons.extend(&scanners[0].beacons);

    let mut pos = vec![];

    while !scanners.is_empty() {
        let (i, orient) = scanners
            .iter()
            .map(|scanner| find_orientation(scanner, &beacons))
            .enumerate()
            .filter_map(|(i, p)| p.map(|p| (i, p)))
            .next()
            .unwrap();

        let scanner = scanners.remove(i);
        beacons.extend(scanner.beacons.iter().map(|&b| orient.inverse(b)));

        let p = orient.rotate.inverse(Beacon {
            x: -orient.shift[0],
            y: -orient.shift[1],
            z: -orient.shift[2],
        });
        pos.push([p.x, p.y, p.z]);
    }

    (beacons, pos)
}

fn largest_manhattan_distance(pos: &[[Num; 3]]) -> Num {
    pos.iter()
        .cartesian_product(pos)
        .map(|([a, b, c], [x, y, z])| (a - x).abs() + (b - y).abs() + (c - z).abs())
        .max()
        .unwrap()
}

pub(crate) fn run(lines: Lines) -> Result {
    let scanners = parse_scanners(lines)?;
    let (beacons, pos) = find_beacons(scanners);

    println!("part A: {:?}", beacons.len());
    println!("part B: {:?}", largest_manhattan_distance(&pos));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        //
    }

    #[test]
    fn test_b() {
        //
    }
}
