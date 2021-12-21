use crate::common::*;

pub(crate) fn parse(lines: Lines) -> Result<[u64; 2]> {
    let a = find_regex("Player 1 starting position: ([0-9])", lines[0])
        .ok_or_else(|| anyhow!("invalid input"))?[1]
        .parse()?;

    let b = find_regex("Player 2 starting position: ([0-9])", lines[1])
        .ok_or_else(|| anyhow!("invalid input"))?[1]
        .parse()?;

    Ok([a, b])
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct GameResult {
    spaces: [u64; 2],
    scores: [u64; 2],
    throws: u64,
    winner: usize,
}

fn play_game(mut spaces: [u64; 2]) -> GameResult {
    let mut throws = 0;
    let mut scores = [0, 0];
    let mut player = 0;

    loop {
        let s = throws % 100 + (throws + 1) % 100 + (throws + 2) % 100 + 3;
        throws += 3;

        spaces[player] = (spaces[player] - 1 + s) % 10 + 1;
        scores[player] += spaces[player];

        if scores[player] >= 1000 {
            break;
        }

        player = (player + 1) % 2;
    }

    GameResult {
        scores,
        throws,
        winner: player,
        spaces,
    }
}

fn play_quantum_game(spaces: [u64; 2]) -> [u64; 2] {
    const MAX_SCORE: usize = 21;
    const MAX_SPACES: usize = 10;

    #[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
    struct State {
        scores: [u8; 2],
        spaces: [u8; 2],
        player: u8,
    }

    fn state_to_key(state: State) -> usize {
        let [a, b] = [state.scores[0] as usize, state.scores[1] as usize];
        let [x, y] = [state.spaces[0] as usize, state.spaces[1] as usize];
        let p = state.player as usize;

        (((a * MAX_SCORE + b) * MAX_SPACES + x) * MAX_SPACES + y) * 2 + p
    }

    let mut cache = vec![[0, 0]; MAX_SCORE * MAX_SCORE * MAX_SPACES * MAX_SPACES * 2];

    fn recur(state: State, cache: &mut [[u64; 2]]) -> [u64; 2] {
        const THROWS: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let key = state_to_key(state);

        if cache[key] != [0, 0] {
            return cache[key];
        }

        let mut result = [0, 0];

        for (total, times) in THROWS {
            let mut state = state;
            let player = state.player as usize;
            state.spaces[player] = (state.spaces[player] + total) % (MAX_SPACES as u8);
            state.scores[player] += state.spaces[player] + 1;
            state.player = 1 - state.player;

            let subresult = if state.scores[player] >= MAX_SCORE as u8 {
                match player {
                    0 => [1, 0],
                    1 => [0, 1],
                    _ => panic!("invalid player"),
                }
            } else {
                recur(state, cache)
            };

            result[0] += times * subresult[0];
            result[1] += times * subresult[1];
        }

        cache[key] = result;
        result
    }

    recur(
        State {
            scores: [0, 0],
            spaces: [spaces[0] as u8 - 1, spaces[1] as u8 - 1],
            player: 0,
        },
        &mut cache,
    )
}

pub(crate) fn run(lines: Lines) -> Result {
    let spaces = parse(lines)?;
    let result = play_game(spaces);

    println!(
        "part A: {:?}",
        result.throws * result.scores[1 - result.winner]
    );

    let result = play_quantum_game(spaces);
    println!("part B: {:?}", u64::max(result[0], result[1]));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let result = play_game([4, 8]);
        assert_eq!(
            result,
            GameResult {
                spaces: [10, 3],
                scores: [1000, 745],
                throws: 993,
                winner: 0,
            }
        );
    }

    #[test]
    fn test_b() {
        let result = play_quantum_game([4, 8]);
        assert_eq!(result, [444356092776315, 341960390180808]);
    }
}
