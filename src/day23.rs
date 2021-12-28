use crate::common::*;
use binary_heap_plus::BinaryHeap;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Amphi {
    A,
    B,
    C,
    D,
}

type State<const N: usize> = Rc<[[Option<Amphi>; 11]; N]>;

fn parse<const N: usize>(lines: Lines) -> Result<State<N>>
where
    [[Option<Amphi>; 11]; N]: Default,
{
    let mut state = <[[Option<Amphi>; 11]; N]>::default();

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            let a = match c {
                'A' => Amphi::A,
                'B' => Amphi::B,
                'C' => Amphi::C,
                'D' => Amphi::D,
                _ => continue,
            };

            state[i - 1][j - 1] = Some(a);
        }
    }

    Ok(Rc::new(state))
}

fn is_solved<const N: usize>(state: &State<N>) -> bool {
    let mut solved = true;

    for i in 1..N {
        for j in 0..4 {
            solved &= match state[i][2 * j + 2] {
                Some(c) => c as usize == j,
                None => false,
            };
        }
    }

    solved
}

fn print_state<const N: usize>(state: &State<N>) {
    use Amphi::*;

    for i in 0..N {
        for j in 0..11 {
            match state[i][j] {
                Some(A) => print!("A"),
                Some(B) => print!("B"),
                Some(C) => print!("C"),
                Some(D) => print!("D"),
                None => print!("."),
            }
        }

        println!();
    }

    println!();
}

fn solve<const N: usize>(initial_state: State<N>) -> usize {
    const HALLWAYS_SPOTS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

    let mut previous = HashMap::<State<N>, State<N>>::default();
    let mut queue =
        BinaryHeap::<(State<N>, State<N>, usize), _>::new_by_key(|&(_, _, cost)| Reverse(cost));
    queue.push((initial_state.clone(), initial_state, 0));

    fn foo<const N: usize>(
        state: &State<N>,
        cost: usize,
        src: [usize; 2],
        dst: [usize; 2],
    ) -> Option<(State<N>, State<N>, usize)> {
        let [mut row, mut col] = src;
        let mut steps = 0;

        while [row, col] != dst {
            steps += 1;

            if row != 0 && col != dst[1] {
                row -= 1; // Move up
            } else if row == 0 && col < dst[1] {
                col += 1; // Move right
            } else if row == 0 && col > dst[1] {
                col -= 1; // Move left
            } else {
                row += 1; // Move down
            }

            if state[row][col].is_some() {
                return None;
            }
        }

        let mut new_state = (**state).clone();
        let me = new_state[src[0]][src[1]].take().unwrap();
        new_state[dst[0]][dst[1]] = Some(me);
        let new_state = Rc::new(new_state);

        let cost_per_step = match me {
            Amphi::A => 1,
            Amphi::B => 10,
            Amphi::C => 100,
            Amphi::D => 1000,
        };

        Some((new_state, state.clone(), cost + cost_per_step * steps))
    }

    while let Some((state, prev, cost)) = queue.pop() {
        match previous.entry(state.clone()) {
            Entry::Vacant(e) => e.insert(prev),
            Entry::Occupied(_) => continue,
        };

        if is_solved(&state) {
            let mut state = state;
            let mut stack = vec![];
            loop {
                stack.push(state.clone());

                let prev = state;
                state = previous[&prev].clone();
                if prev == state {
                    break;
                }
            }

            for x in rev(stack) {
                print_state(&x);
            }

            return cost;
        }

        let mut moved_into = false;

        // Try to move into a room
        'into: for src in HALLWAYS_SPOTS {
            let c = match state[0][src] {
                Some(c) => c,
                None => continue,
            };

            let col = 2 * (c as usize) + 2;
            let mut row = N - 1;

            loop {
                match state[row][col] {
                    None => break,
                    Some(x) if x == c && row > 1 => row -= 1,
                    _ => continue 'into,
                }
            }

            if let Some(x) = foo(&state, cost, [0, src], [row, col]) {
                moved_into = true;
                queue.push(x);
            }
        }

        // Try to move out of room
        if !moved_into {
            'out: for i in 0..4 {
                let col = 2 * i + 2;
                let mut row = 1;

                while state[row][col].is_none() {
                    if row + 1 < N {
                        row += 1;
                    } else {
                        continue 'out;
                    }
                }

                for target in HALLWAYS_SPOTS {
                    if let Some(x) = foo(&state, cost, [row, col], [0, target]) {
                        queue.push(x);
                    }
                }
            }
        }
    }

    panic!("no solution found!");
}

pub(crate) fn run(lines: Lines) -> Result {
    let state = parse::<3>(lines)?;
    println!("part A: {}", solve(state));

    let state = parse::<5>(&[
        lines[0],
        lines[1],
        lines[2],
        "  #D#C#B#A#",
        "  #D#B#A#C#",
        lines[3],
        lines[4],
    ])?;
    println!("part B: {}", solve(state));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let lines = [
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #A#D#C#A#",
            "  #########",
        ];

        assert_eq!(solve(parse::<3>(&lines).unwrap()), 12521);
    }

    #[test]
    fn test_b() {
        let lines = [
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #D#C#B#A#",
            "  #D#B#A#C#",
            "  #A#D#C#A#",
            "  #########",
        ];

        assert_eq!(solve(parse::<5>(&lines).unwrap()), 44169);
    }
}
