use crate::common::*;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
enum Var {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}
type Num = i64;
type State = [Num; 4];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Arg {
    Var(Var),
    Const(Num),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Expr {
    Input(Var),
    Add(Var, Arg),
    Mul(Var, Arg),
    Div(Var, Arg),
    Mod(Var, Arg),
    Eq(Var, Arg),
}

fn parse_expr(line: &str) -> Result<Expr> {
    use Expr::*;

    fn parse_var(name: &str) -> Result<Var> {
        Ok(match name {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _ => bail!("invalid variable: {:?}", name),
        })
    }

    let mut parts = line.split_whitespace();
    let op = parts
        .next()
        .ok_or_else(|| anyhow!("invalid input: no opcode"))?;
    let lhs = parts
        .next()
        .ok_or_else(|| anyhow!("invalid input: no first argument"))
        .and_then(parse_var)?;

    if op == "inp" {
        return Ok(Expr::Input(lhs));
    }

    let part = parts
        .next()
        .ok_or_else(|| anyhow!("invalid input: no second argument"))?;

    let rhs = if let Ok(x) = part.parse::<Num>() {
        Arg::Const(x)
    } else {
        Arg::Var(parse_var(part)?)
    };

    Ok(match op {
        "add" => Add(lhs, rhs),
        "mul" => Mul(lhs, rhs),
        "div" => Div(lhs, rhs),
        "mod" => Mod(lhs, rhs),
        "eql" => Eq(lhs, rhs),
        _ => bail!("invalid input: unknown opcode: {:?}", op),
    })
}

fn parse(lines: Lines) -> Result<Vec<Expr>> {
    lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| parse_expr(l))
        .collect()
}

fn reorder_instructions(instr: &[Expr]) -> Vec<Expr> {
    const VARS: [Var; 4] = [Var::W, Var::X, Var::Y, Var::Z];

    use Expr::*;
    let mut lines = VecDeque::new();
    let mut active = [true; 4];

    for &line in rev(instr) {
        for v in VARS {
            let is_read = match line {
                Add(a, _) | Mul(a, _) | Div(a, _) | Mod(a, _) | Eq(a, _) if a == v => true,
                Add(_, a) | Mul(_, a) | Div(_, a) | Mod(_, a) | Eq(_, a) if a == Arg::Var(v) => {
                    true
                }
                _ => false,
            };

            if is_read && !active[v as usize] {
                lines.push_front(Mul(v, Arg::Const(0)));
                active[v as usize] = true;
            }
        }

        lines.push_front(line);

        for v in VARS {
            if matches!(line, Input(w) | Mul(w, Arg::Const(0)) if w == v) {
                active[v as usize] = false;
            }
        }
    }

    lines.into()
}

fn execute<A>(instr: &[Expr], arbiter: A) -> Num
where
    A: Fn(Num, Num) -> Num,
{
    use Expr::*;

    fn evolve<'a>(
        instr: &'a [Expr],
        iter: impl IntoIterator<Item = (State, Num)> + 'a,
    ) -> impl Iterator<Item = (State, Num)> + 'a {
        iter.into_iter()
            .cartesian_product(1..=9)
            .map(|((state, num), i)| {
                let new_state = eval(instr, &[i], state);
                let new_num = 10 * num + i;
                (new_state, new_num)
            })
    }

    let (prelude, mut instr) = match instr.iter().position(|e| matches!(e, Input(_))) {
        Some(i) => instr.split_at(i),
        None => (instr, &[][..]),
    };

    let mut new_states = HashMap::default();
    let mut states = HashMap::default();
    let initial_state = eval(prelude, &[], default());
    states.insert(initial_state, 0);

    while let Some(split) = instr.iter().skip(1).position(|e| matches!(e, Input(_))) {
        let (head, rest) = instr.split_at(split + 1);
        instr = rest;

        new_states.clear();

        for (state, num) in evolve(head, states.drain()) {
            new_states
                .entry(state)
                .and_modify(|v| *v = arbiter(*v, num))
                .or_insert(num);
        }

        swap(&mut states, &mut new_states);
    }

    evolve(instr, states.drain())
        .filter_map(|(state, num)| (state[Var::Z as usize] == 0).then(|| num))
        .reduce(arbiter)
        .unwrap()
}

#[inline(always)]
fn eval(lines: &[Expr], mut inputs: &[Num], mut state: State) -> State {
    use Expr::*;

    fn evolve<F>(out: Var, arg: Arg, fun: F, state: &mut State)
    where
        F: Fn(Num, Num) -> Num,
    {
        let lhs = state[out as usize];
        let rhs = match arg {
            Arg::Var(i) => state[i as usize],
            Arg::Const(c) => c,
        };

        state[out as usize] = fun(lhs, rhs);
    }

    for &line in lines {
        match line {
            Input(v) => {
                let (&first, rest) = inputs.split_first().unwrap();
                state[v as usize] = first;
                inputs = rest;
            }
            Add(a, b) => evolve(a, b, |x, y| x + y, &mut state),
            Mul(a, b) => evolve(a, b, |x, y| x * y, &mut state),
            Div(a, b) => evolve(a, b, |x, y| x / y, &mut state),
            Mod(a, b) => evolve(a, b, |x, y| x % y, &mut state),
            Eq(a, b) => evolve(a, b, |x, y| (x == y) as _, &mut state),
        }
    }

    state
}

pub(crate) fn run(lines: Lines) -> Result {
    let lines = parse(lines)?;
    let lines = reorder_instructions(&lines);

    println!("part A: {:?}", execute(&lines, |a, b| Num::max(a, b)));
    println!("part B: {:?}", execute(&lines, |a, b| Num::min(a, b)));
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
