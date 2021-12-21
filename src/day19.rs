use crate::common::*;
use recap::Recap;
use serde::Deserialize;

type Num = i64;
type Vec3 = nalgebra::Vector3<Num>;
type Mat3 = nalgebra::Matrix3<Num>;

#[derive(Hash, Debug, Deserialize, PartialEq, Recap, Copy, Clone, Ord, PartialOrd, Eq)]
#[recap(regex = r#"(?P<x>-?[0-9]+),(?P<y>-?[0-9]+),(?P<z>-?[0-9]+)"#)]
struct Beacon {
    x: Num,
    y: Num,
    z: Num,
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Vec3>,
}

fn parse_scanners(lines: Lines) -> Result<Vec<Scanner>> {
    let mut lines = lines.iter().peekable();
    let mut scanners = vec![];

    while let Some(header) = lines.next() {
        if !is_match("--- scanner [0-9]+ ---", header) {
            bail!("invalid header: {:?}", header);
        }

        let beacons = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| Ok(l.parse::<Beacon>()?))
            .map_ok(|p| Vec3::new(p.x, p.y, p.z))
            .collect::<Result<_>>()?;

        scanners.push(Scanner { beacons });
    }

    Ok(scanners)
}

fn rotations() -> Vec<Mat3> {
    const VECTORS: [[Num; 3]; 6] = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ];

    let mut result = vec![];

    for dx in VECTORS {
        for dy in VECTORS {
            let dz = [
                dx[1] * dy[2] - dx[2] * dy[1],
                dx[2] * dy[0] - dx[0] * dy[2],
                dx[0] * dy[1] - dx[1] * dy[0],
            ];

            if dz[0].abs() + dz[1].abs() + dz[2].abs() != 1 {
                continue;
            }

            result.push(Mat3::new(
                dx[0], dx[1], dx[2], //
                dy[0], dy[1], dy[2], //
                dz[0], dz[1], dz[2], //
            ));
        }
    }

    result
}

fn align_scanners(scanners: &[Scanner]) -> Vec<(Mat3, Vec3)> {
    let rotations = rotations();
    let mut count: HashMap<Vec3, usize> = HashMap::default();
    let mut results = vec![];

    for (j, b) in enumerate(scanners) {
        for &r in &rotations {
            let rotated = map(&b.beacons, |x| r * x).collect_vec();

            for (i, a) in enumerate(scanners) {
                count.clear();

                for &x in &a.beacons {
                    for &y in &rotated {
                        *count.entry(x - y).or_default() += 1;
                    }
                }

                for (&t, &freq) in &count {
                    if freq >= 12 {
                        results.push((i, j, r, t));
                    }
                }
            }
        }
    }

    let n = scanners.len();
    let mut valid = vec![false; n];
    let mut orients = vec![(Mat3::identity(), Vec3::default()); n];
    valid[0] = true;

    for _ in 0..n {
        for &(i, j, r, t) in &results {
            if valid[i] && !valid[j] {
                valid[j] = true;
                orients[j] = (
                    orients[i].0 * r,                //
                    orients[i].0 * t + orients[i].1, //
                );
            }
        }
    }

    orients
}

fn find_beacons(scanners: &[Scanner], orientations: &[(Mat3, Vec3)]) -> Vec<Vec3> {
    zip(scanners, orientations)
        .flat_map(|(scanner, (r, t))| map(&scanner.beacons, move |b| r * b + t))
        .unique()
        .collect()
}

fn largest_distance(orientations: &[(Mat3, Vec3)]) -> Num {
    let mut largest = 0;

    for (_, t1) in orientations {
        for (_, t2) in orientations {
            largest = (t1 - t2).abs().sum().max(largest);
        }
    }

    largest
}

pub(crate) fn run(lines: Lines) -> Result {
    let scanners = parse_scanners(lines)?;
    let orients = align_scanners(&scanners);

    println!("part A: {:?}", find_beacons(&scanners, &orients).len());
    println!("part B: {:?}", largest_distance(&orients));

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
