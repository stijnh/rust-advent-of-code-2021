mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

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
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
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
