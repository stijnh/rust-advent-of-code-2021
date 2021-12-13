use crate::common::*;

struct SyntaxError {
    character: char,
}

fn parse(line: &str) -> Result<Vec<char>, SyntaxError> {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            '(' => stack.push(')'),
            c if stack.pop() == Some(c) => {}
            _ => return Err(SyntaxError { character: c }),
        }
    }

    Ok(stack)
}

fn solve_a(lines: Lines) -> Result<usize> {
    let mut sum = 0;

    for line in lines {
        if let Err(SyntaxError { character: c }) = parse(line) {
            sum += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => bail!("unknown character: {:?}", c),
            }
        }
    }

    Ok(sum)
}

fn solve_b(lines: Lines) -> Result<usize> {
    let mut scores = vec![];

    for line in lines {
        if let Ok(stack) = parse(line) {
            let mut score = 0;

            for c in rev(stack) {
                score = score * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("invalid character: {:?}", c),
                    }
            }

            scores.push(score);
        }
    }

    scores.sort();
    Ok(scores[scores.len() / 2])
}

pub(crate) fn run(lines: Lines) -> Result {
    println!("part A: {:?}", solve_a(&lines)?);
    println!("part B: {:?}", solve_b(&lines)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static [&'static str] {
        &[
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
    }

    #[test]
    fn test_a() {
        assert_eq!(solve_a(input()).unwrap(), 26397);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(input()).unwrap(), 288957);
    }
}
