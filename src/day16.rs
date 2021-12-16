use crate::common::*;
use std::str::Chars;

type Num = u64;

#[derive(Clone)]
struct BitStream<'a> {
    index: usize,
    input: Chars<'a>,
    buffer: [bool; 4],
    buffer_index: usize,
}

impl<'a> BitStream<'a> {
    fn new(input: &'a str) -> Result<Self> {
        for c in input.chars() {
            if !c.is_ascii_hexdigit() {
                bail!("invalid input: {:?}", input);
            }
        }

        Ok(Self {
            index: 0,
            input: input.chars(),
            buffer: [false; 4],
            buffer_index: 4,
        })
    }

    fn index(&self) -> usize {
        self.index
    }

    fn next_number(&mut self, bits: usize) -> Option<Num> {
        let mut output = 0;

        for _ in 0..bits {
            output = (output << 1) | (self.next()? as Num);
        }

        Some(output)
    }
}

impl Iterator for BitStream<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_index >= self.buffer.len() {
            let c = self.input.next()?.to_digit(16).unwrap();

            for i in 0..4 {
                self.buffer[i] = (c & (0x8 >> i)) != 0;
            }

            self.buffer_index = 0;
        }

        let output = self.buffer[self.buffer_index];
        self.buffer_index += 1;
        self.index += 1;
        return Some(output);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Packet {
    version: Num,
    typeid: Num,
    content: Content,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Content {
    Literal(Num),
    Sequence(Vec<Packet>),
}

fn parse(stream: &mut BitStream) -> Result<Packet> {
    let version = stream
        .next_number(3)
        .ok_or_else(|| anyhow!("invalid version"))?;
    let typeid = stream
        .next_number(3)
        .ok_or_else(|| anyhow!("invalid typeid"))?;

    let content = if typeid == 4 {
        let mut x = 0;
        loop {
            let continuation = stream.next().ok_or_else(|| anyhow!("invalid flag"))?;
            let y = stream
                .next_number(4)
                .ok_or_else(|| anyhow!("invalid chunk"))?;
            x = (x << 4) | y;

            if !continuation {
                break;
            }
        }

        Content::Literal(x)
    } else {
        let lengthid = stream.next().ok_or_else(|| anyhow!("invalid lengthid"))?;
        let mut subpackets = vec![];

        if lengthid {
            let n = stream
                .next_number(11)
                .ok_or_else(|| anyhow!("invalid length"))?;

            for _ in 0..n {
                subpackets.push(parse(stream)?);
            }
        } else {
            let n = stream
                .next_number(15)
                .ok_or_else(|| anyhow!("invalid length"))? as usize;
            let start = stream.index();

            while stream.index() < start + n {
                subpackets.push(parse(stream)?);
            }

            assert_eq!(stream.index(), start + n);
        }

        Content::Sequence(subpackets)
    };

    Ok(Packet {
        version,
        typeid,
        content,
    })
}

fn sum_versions(packet: &Packet) -> Num {
    let mut output = packet.version as _;

    match &packet.content {
        Content::Sequence(children) => {
            for child in children {
                output += sum_versions(&child);
            }
        }
        Content::Literal(_) => {}
    }

    output
}

fn eval(packet: &Packet) -> Result<Num> {
    let children = match &packet.content {
        &Content::Literal(x) => return Ok(x as _),
        Content::Sequence(children) => map(children, |x| eval(x)).collect::<Result<Vec<_>>>()?,
    };

    Ok(match packet.typeid {
        0 => children.into_iter().sum(),
        1 => children.into_iter().product(),
        2 => children.into_iter().min().unwrap_or_default(),
        3 => children.into_iter().max().unwrap_or_default(),
        5 => (children[0] > children[1]) as _,
        6 => (children[0] < children[1]) as _,
        7 => (children[0] == children[1]) as _,
        v => bail!("unknown version: {:?}", v),
    })
}

pub(crate) fn run(lines: Lines) -> Result {
    let p = parse(&mut BitStream::new(lines[0])?)?;

    println!("part A: {:?}", sum_versions(&p));
    println!("part B: {:?}", eval(&p)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(input: &str) -> Packet {
        parse(&mut BitStream::new(input).unwrap()).unwrap()
    }

    #[test]
    fn test_a() {
        let packet = p("D2FE28");
        assert_eq!(
            packet,
            Packet {
                version: 6,
                typeid: 4,
                content: Content::Literal(2021),
            }
        );

        let packet = p("38006F45291200");
        assert_eq!(
            packet,
            Packet {
                version: 1,
                typeid: 6,
                content: Content::Sequence(vec![
                    Packet {
                        version: 6,
                        typeid: 4,
                        content: Content::Literal(10),
                    },
                    Packet {
                        version: 2,
                        typeid: 4,
                        content: Content::Literal(20),
                    },
                ]),
            }
        );

        let packet = p("EE00D40C823060");
        assert_eq!(
            packet,
            Packet {
                version: 7,
                typeid: 3,
                content: Content::Sequence(vec![
                    Packet {
                        version: 2,
                        typeid: 4,
                        content: Content::Literal(1),
                    },
                    Packet {
                        version: 4,
                        typeid: 4,
                        content: Content::Literal(2),
                    },
                    Packet {
                        version: 1,
                        typeid: 4,
                        content: Content::Literal(3),
                    },
                ]),
            }
        );

        assert_eq!(sum_versions(&p("8A004A801A8002F478")), 16);
        assert_eq!(sum_versions(&p("620080001611562C8802118E34")), 12);
        assert_eq!(sum_versions(&p("C0015000016115A2E0802F182340")), 23);
        assert_eq!(sum_versions(&p("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_b() {
        assert_eq!(eval(&p("C200B40A82")).unwrap(), 3);
        assert_eq!(eval(&p("04005AC33890")).unwrap(), 54);
        assert_eq!(eval(&p("880086C3E88112")).unwrap(), 7);
        assert_eq!(eval(&p("CE00C43D881120")).unwrap(), 9);
        assert_eq!(eval(&p("D8005AC2A8F0")).unwrap(), 1);
        assert_eq!(eval(&p("F600BC2D8F")).unwrap(), 0);
        assert_eq!(eval(&p("9C005AC2F8F0")).unwrap(), 0);
        assert_eq!(eval(&p("9C0141080250320F1802104A08")).unwrap(), 1);
    }
}
