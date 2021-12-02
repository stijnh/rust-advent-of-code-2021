mod common;
mod day01;
mod day02;

use common::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result {
    let funs = [day01::run, day02::run];

    let mut args = env::args();
    let binary = args.next().unwrap_or_default();
    let day = args.next().unwrap_or_default();

    let day = if let Ok(i) = day.parse::<usize>() {
        i
    } else {
        bail!("usage: {} [day]", binary);
    };

    if day <= 0 || day > funs.len() {
        bail!("day must be a number between 1 and {}", funs.len());
    }

    let input_file = format!("inputs/day{:02}", day);
    let f = File::open(&input_file).with_context(|| format!("failed to open: {}", input_file))?;

    let lines = BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .with_context(|| format!("error while reading: {}", input_file))?;

    (funs[day - 1])(lines)
}
