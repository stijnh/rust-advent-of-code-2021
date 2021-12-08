use crate::common::*;

lazy_static::lazy_static! {
    static ref DIGITS: [Sample; 10] = {
        ["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]
            .iter()
            .map(|e| parse_sample(e).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    };
}

type Sample = [bool; 7];

struct Entry {
    inputs: [Sample; 10],
    outputs: [Sample; 4],
}

fn parse_sample(chars: &str) -> Result<Sample> {
    let mut sample: Sample = default();

    for c in chars.chars() {
        let num = u32::wrapping_sub(c as u32, 'a' as u32) as usize;
        if num >= sample.len() {
            bail!("invalid character: {:?}", c);
        }

        sample[num] = true;
    }

    Ok(sample)
}

fn parse_entry(line: &str) -> Result<Entry> {
    let err = || anyhow!("invalid input: {:?}", line);
    let mut iter = line.split_whitespace();
    let mut inputs: [Sample; 10] = default();
    let mut outputs: [Sample; 4] = default();

    for v in &mut inputs {
        *v = parse_sample(iter.next().ok_or_else(err)?)?;
    }

    if iter.next() != Some("|") {
        bail!(err());
    }

    for v in &mut outputs {
        *v = parse_sample(iter.next().ok_or_else(err)?)?;
    }

    Ok(Entry { inputs, outputs })
}

fn parse(lines: Lines) -> Result<Vec<Entry>> {
    lines.iter().map(|s| parse_entry(s)).collect()
}

fn count_digits(input: &[bool]) -> usize {
    input.iter().map(|&b| b as usize).sum()
}

fn solve_a(entries: &[Entry]) -> usize {
    let mut sum = 0;

    for entry in entries {
        for output in entry.outputs {
            if matches!(count_digits(&output), 2 | 3 | 4 | 7) {
                sum += 1;
            }
        }
    }

    sum
}

type Mapping = [usize; 7];

#[allow(clippy::needless_range_loop)]
fn find_mapping(entry: &Entry) -> Mapping {
    let mut table = [[true; 7]; 7];

    for src in 0..7 {
        let count = entry.inputs.iter().map(|s| s[src] as usize).sum();

        let dst = match count {
            4 => 'e',
            9 => 'f',
            6 => 'b',
            _ => continue,
        };
        let dst = (dst as u32 - 'a' as u32) as usize;

        for i in 0..7 {
            for j in 0..7 {
                if (i == src) ^ (j == dst) {
                    table[i][j] = false;
                }
            }
        }
    }

    for src in entry.inputs {
        let dst = match count_digits(&src) {
            2 => DIGITS[1],
            3 => DIGITS[7],
            4 => DIGITS[4],
            _ => continue,
        };

        for i in 0..7 {
            for j in 0..7 {
                if src[i] ^ dst[j] {
                    table[i][j] = false;
                }
            }
        }
    }

    let mut mapping = Mapping::default();
    let mut found = 0;

    for i in 0..7 {
        for j in 0..7 {
            if table[i][j] {
                mapping[i] = j;
                found += 1;
            }
        }
    }

    if found != 7 {
        panic!("invalid input");
    }

    mapping
}

fn decode_digit(encoded: Sample, mapping: Mapping) -> usize {
    let mut decoded = Sample::default();

    for (i, &active) in enumerate(&encoded) {
        if active {
            decoded[mapping[i]] = true;
        }
    }

    for (i, &digit) in enumerate(&*DIGITS) {
        if digit == decoded {
            return i;
        }
    }

    panic!("unknown digit: {:?}", decoded);
}

fn decode_output(entry: &Entry, mapping: Mapping) -> usize {
    let mut result = 0;

    for output in entry.outputs {
        result = result * 10 + decode_digit(output, mapping);
    }

    result
}

fn solve_b(entries: &[Entry]) -> usize {
    let mut sum = 0;

    for entry in entries {
        let mapping = find_mapping(entry);
        sum += decode_output(entry, mapping);
    }

    sum
}

pub(crate) fn run(lines: Lines) -> Result {
    let entries = parse(lines)?;

    println!("part A: {:?}", solve_a(&entries));
    println!("part B: {:?}", solve_b(&entries));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Entry> {
        let lines = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ];

        parse(&lines).unwrap()
    }

    #[test]
    fn test_a() {
        assert_eq!(solve_a(&input()), 26);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(&input()), 61229);
    }
}
