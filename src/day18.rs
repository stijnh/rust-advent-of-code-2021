use crate::common::*;
use std::fmt::{self, Display};
use std::str::Chars;

type Num = i64;

#[derive(Clone)]
enum SnailNum {
    Value(Num),
    Pair(Box<SnailNum>, Box<SnailNum>),
}

use SnailNum::*;

impl Display for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value(v) => write!(f, "{}", v),
            Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

fn parse_line(line: &mut Chars<'_>) -> Result<SnailNum> {
    match line.next().unwrap_or_default() {
        '[' => {
            let left = parse_line(line)?;
            ensure!(line.next() == Some(','), "expecting ','");
            let right = parse_line(line)?;
            ensure!(line.next() == Some(']'), "expecting ']'");
            Ok(Pair(Box::new(left), Box::new(right)))
        }
        c @ '0'..='9' => Ok(Value(c as i64 - '0' as i64)),
        other => {
            bail!("invalid character: {:?}", other)
        }
    }
}

fn parse_lines(lines: &[&str]) -> Result<Vec<SnailNum>> {
    lines
        .iter()
        .map(|line| parse_line(&mut line.chars()))
        .collect()
}

fn apply_explode(num: &mut SnailNum) -> bool {
    enum Action {
        Left(Num),
        Right(Num),
    }
    use Action::*;

    enum Return {
        Explode(Num, Num),
        Nothing,
    }
    use Return::*;

    fn apply(num: &mut SnailNum, action: Action) {
        match (num, action) {
            (_, Left(0) | Right(0)) => {}
            (Value(v), Left(x) | Right(x)) => *v += x,
            (Pair(l, _), Left(x)) => apply(l, Left(x)),
            (Pair(_, r), Right(x)) => apply(r, Right(x)),
        }
    }

    fn recur(num: &mut SnailNum, depth: u32) -> Return {
        match num {
            Pair(l, r) if depth > 3 => {
                if let (&Value(x), &Value(y)) = (&**l, &**r) {
                    *num = Value(0);
                    Explode(x, y)
                } else {
                    panic!("invalid snail number");
                }
            }
            Pair(l, r) => {
                if let Explode(x, y) = recur(l, depth + 1) {
                    apply(r, Left(y));
                    Explode(x, 0)
                } else if let Explode(x, y) = recur(r, depth + 1) {
                    apply(l, Right(x));
                    Explode(0, y)
                } else {
                    Nothing
                }
            }
            Value(_) => Nothing,
        }
    }

    matches!(recur(num, 0), Explode(_, _))
}

fn apply_split(num: &mut SnailNum) -> bool {
    match *num {
        Pair(ref mut l, ref mut r) => apply_split(l) || apply_split(r),
        Value(v) if v >= 10 => {
            let half = v / 2;
            *num = Pair(Box::new(Value(half)), Box::new(Value(v - half)));
            true
        }
        Value(_) => false,
    }
}

fn reduce(num: &mut SnailNum) {
    while apply_explode(num) || apply_split(num) {}
}

fn magnitude(num: &SnailNum) -> Num {
    match num {
        Value(v) => *v,
        Pair(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}

fn add(left: SnailNum, right: SnailNum) -> SnailNum {
    let mut output = Pair(Box::new(left), Box::new(right));
    reduce(&mut output);
    output
}

fn sum(numbers: &[SnailNum]) -> SnailNum {
    let mut output = numbers[0].clone();

    for number in &numbers[1..] {
        output = add(output, number.clone());
    }

    output
}

fn largest_sum(numbers: &[SnailNum]) -> SnailNum {
    numbers
        .iter()
        .cartesian_product(numbers)
        .map(|(l, r)| add(l.clone(), r.clone()))
        .max_by_key(magnitude)
        .unwrap()
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = parse_lines(lines)?;

    println!("part A: {}", magnitude(&sum(&numbers)));
    println!("part A: {}", magnitude(&largest_sum(&numbers)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut inputs = parse_lines(&[
            "[[[[[9,8],1],2],3],4]",
            "[7,[6,[5,[4,[3,2]]]]]",
            "[[6,[5,[4,[3,2]]]],1]",
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[1,2],[[3,4],5]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ])
        .unwrap();

        for x in &mut inputs {
            reduce(x);
        }

        assert_eq!(inputs[0].to_string(), "[[[[0,9],2],3],4]");
        assert_eq!(inputs[1].to_string(), "[7,[6,[5,[7,0]]]]");
        assert_eq!(inputs[2].to_string(), "[[6,[5,[7,0]]],3]");
        assert_eq!(inputs[3].to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        assert_eq!(magnitude(&inputs[4]), 143);
        assert_eq!(magnitude(&inputs[5]), 1384);
        assert_eq!(magnitude(&inputs[6]), 445);
        assert_eq!(magnitude(&inputs[7]), 791);
        assert_eq!(magnitude(&inputs[8]), 1137);
        assert_eq!(magnitude(&inputs[9]), 3488);
    }

    #[test]
    fn test_b() {
        //
    }
}
