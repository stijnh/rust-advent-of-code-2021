use crate::common::*;

type Map = HashMap<String, Vec<String>>;

fn parse(lines: Lines) -> Result<Map> {
    let mut output: Map = default();

    for line in lines {
        let (x, y) = line
            .split_once('-')
            .ok_or_else(|| anyhow!("invalid input"))?;
        output.entry(x.to_string()).or_default().push(y.to_string());
        output.entry(y.to_string()).or_default().push(x.to_string());
    }

    Ok(output)
}

fn is_small(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_lowercase())
}

fn count_paths(map: &Map, double_allowed: bool) -> usize {
    let mut stack = vec![];
    let mut count = 0;

    struct State<'a> {
        current: &'a str,
        visited: Vec<&'a str>,
        visited_twice: bool,
    }

    stack.push(State {
        current: "start",
        visited: vec![],
        visited_twice: !double_allowed,
    });

    while let Some(state) = stack.pop() {
        if state.current == "end" {
            count += 1;

            if double_allowed {
                continue;
            }
        }

        for neighbor in &map[state.current] {
            if neighbor == "start" {
                continue;
            }

            let mut visited = state.visited.clone();
            let mut visited_twice = state.visited_twice;

            if is_small(&neighbor) {
                if visited.contains(&&**neighbor) {
                    if !visited_twice {
                        visited_twice = true;
                    } else {
                        continue;
                    }
                }

                visited.push(neighbor);
            }

            stack.push(State {
                current: neighbor,
                visited,
                visited_twice,
            });
        }
    }

    count
}

pub(crate) fn run(lines: Lines) -> Result {
    let map = parse(lines)?;

    println!("part A: {:?}", count_paths(&map, false));
    println!("part A: {:?}", count_paths(&map, true));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Map {
        let lines = [
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(count_paths(&input(), false), 19);
    }

    #[test]
    fn test_b() {
        assert_eq!(count_paths(&input(), true), 103);
    }
}
