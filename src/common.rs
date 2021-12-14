pub use anyhow::{anyhow, bail, ensure, Context as _, Error};
pub use itertools::{all, any, enumerate, max, min, rev, zip, Itertools};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Ord, Ordering};
use std::default::Default;
use std::fmt::Display;
use std::iter::{Flatten, Map, Sum};
pub use std::mem::swap;
use std::str::FromStr;
use std::sync::Mutex;

pub type HashMap<K, V> = std::collections::HashMap<K, V, fnv::FnvBuildHasher>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
pub type Lines<'a> = &'a [&'a str];

#[allow(dead_code)]
pub fn default<T: Default>() -> T {
    T::default()
}

#[allow(dead_code)]
pub fn cmp<T: Ord>(lhs: T, rhs: T) -> Ordering {
    Ord::cmp(&lhs, &rhs)
}

#[allow(dead_code)]
pub fn map<I, F, B>(iter: I, fun: F) -> Map<I::IntoIter, F>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> B,
{
    iter.into_iter().map(fun)
}

#[allow(dead_code)]
pub fn find<I, F>(iter: I, fun: F) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    iter.into_iter().find(fun)
}

#[allow(dead_code)]
pub fn sum<I>(iter: I) -> I::Item
where
    I: IntoIterator,
    I::Item: Sum,
{
    iter.into_iter().sum()
}

#[allow(dead_code)]
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    iter.into_iter().flatten()
}

pub fn parse_list<I: FromStr>(line: &str, delim: char) -> Result<Vec<I>>
where
    I::Err: Display,
{
    line.split(delim)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| match s.parse() {
            Ok(v) => Ok(v),
            Err(e) => bail!("failed to parse {:?}: {}", s, e),
        })
        .collect()
}

lazy_static! {
    static ref PATTERN_CACHE: Mutex<HashMap<String, &'static Regex>> = Mutex::default();
}

fn compile(pattern: &str) -> &'static Regex {
    let mut guard = PATTERN_CACHE.lock().unwrap();
    if let Some(p) = guard.get(pattern) {
        return p;
    }

    let result = Box::leak(Box::new(Regex::new(pattern).unwrap()));
    guard.insert(pattern.to_string(), result);

    result
}

#[allow(dead_code)]
pub fn is_match(pattern: &str, string: &str) -> bool {
    compile(pattern).is_match(string)
}

#[allow(dead_code)]
pub fn find_regex<'t>(pattern: &str, string: &'t str) -> Option<regex::Captures<'t>> {
    compile(pattern).captures(string)
}

#[allow(dead_code)]
pub fn find_regex_all<'t>(pattern: &str, string: &'t str) -> regex::CaptureMatches<'static, 't> {
    compile(pattern).captures_iter(string)
}
