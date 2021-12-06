use crate::common::*;
use ndarray::{Array2, ArrayView2};

const N: usize = 5;
type Num = i32;
type BingoCard = Array2<Num>;

fn parse_numbers(line: &str) -> Result<Vec<Num>> {
    line.split(",")
        .filter(|s| !s.is_empty())
        .map(|s| Ok(s.parse()?))
        .collect()
}

fn parse_cards(lines: &[String]) -> Result<Vec<BingoCard>> {
    let mut cards = vec![];
    let mut lines = lines.iter();

    loop {
        let mut card = Array2::zeros((N, N));

        for i in 0..N {
            let line = lines.next();
            let line = line.context("invalid input")?;

            for (j, s) in enumerate(line.split_whitespace()) {
                card[[i, j]] = s.parse().context("invalid number")?;
            }
        }

        cards.push(card);

        if lines.next().is_none() {
            break;
        }
    }

    Ok(cards)
}

fn has_bingo(checked: ArrayView2<bool>) -> bool {
    (0..N).any(|i| (0..N).all(|j| checked[[i, j]]))
        || (0..N).any(|i| (0..N).all(|j| checked[[j, i]]))
}

fn play_card(numbers: &[Num], card: &BingoCard) -> Option<(usize, Num)> {
    let mut checked: Array2<bool> = Array2::from_elem((N, N), false);

    for (turn, &x) in enumerate(numbers) {
        zip(&mut checked, card)
            .filter(|(_, &y)| x == y)
            .for_each(|(c, _)| *c = true);

        if has_bingo(checked.view()) {
            let score = zip(&checked, card)
                .filter(|(&c, _)| !c)
                .map(|(_, y)| y)
                .sum();

            return Some((turn, score));
        }
    }

    None
}

fn play_cards_winner(numbers: &[Num], cards: &[BingoCard]) -> Num {
    let (turn, score) = cards
        .iter()
        .filter_map(|card| play_card(numbers, card))
        .min_by_key(|&(turn, _)| turn)
        .unwrap();

    score * numbers[turn]
}

fn play_cards_loser(numbers: &[Num], cards: &[BingoCard]) -> Num {
    let (turn, score) = cards
        .iter()
        .filter_map(|card| play_card(numbers, card))
        .max_by_key(|&(turn, _)| turn)
        .unwrap();

    score * numbers[turn]
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = parse_numbers(&lines[0])?;
    let cards = parse_cards(&lines[2..])?;

    let answer = play_cards_winner(&numbers, &cards);
    println!("part A: {:?}", answer);

    let answer = play_cards_loser(&numbers, &cards);
    println!("part B: {:?}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> (Vec<Num>, Vec<BingoCard>) {
        let line = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
        let lines = [
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect_vec();

        (parse_numbers(line).unwrap(), parse_cards(&lines).unwrap())
    }

    #[test]
    fn test_a() {
        let (numbers, cards) = input();

        let answer = play_cards_winner(&numbers, &cards);
        assert_eq!(answer, 4512);
    }

    #[test]
    fn test_b() {
        let (numbers, cards) = input();

        let answer = play_cards_loser(&numbers, &cards);
        assert_eq!(answer, 1924);
    }
}
