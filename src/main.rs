mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use common::*;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result {
    let funs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
    ];

    let mut args = env::args();
    let binary = args.next().unwrap_or_default();
    let day = args.next().unwrap_or_default();

    let day = if let Ok(i) = day.parse::<usize>() {
        i
    } else {
        bail!("usage: {} [day]", binary);
    };

    if day == 0 || day > funs.len() {
        bail!("day must be a number between 1 and {}", funs.len());
    }

    let mut input_file = String::new();

    for &prefix in &[".", "..", "inputs", "../inputs/"] {
        input_file = format!("{}/day{:02}", prefix, day);

        if Path::new(&input_file).exists() {
            break;
        }
    }

    let content =
        read_to_string(&input_file).with_context(|| format!("failed to open: {}", input_file))?;
    let lines = content.trim().split('\n').collect::<Vec<_>>();

    (funs[day - 1])(&lines)
}
