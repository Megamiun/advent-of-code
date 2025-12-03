extern crate core;
extern crate derive_more;
extern crate forward_ref_generic;
extern crate ibig;
extern crate itertools;
extern crate paste;
extern crate regex;
#[cfg(test)]
extern crate rstest;
extern crate rustc_hash;

use itertools::Itertools;
use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;

mod y2023;
mod y2024;
mod y2025;
pub mod util;
pub mod testing;

pub fn run_for_n_group<T: Display, const N: usize>(year: u32, day: u32, solution: &str, file: &str, exec: impl Fn(&[&[String]; N]) -> T) -> T {
    run_for_file(year, day, solution, file, |lines| {
        let groups = lines.splitn(N, |line| line.is_empty()).collect_vec();
        exec(&groups.try_into().unwrap())
    })
}

pub fn run_for_group<T: Display>(year: u32, day: u32, solution: &str, file: &str, exec: impl Fn(&[&[String]]) -> T) -> T {
    run_for_file(year, day, solution, file, |lines| {
        let groups = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
        exec(groups.as_slice())
    })
}

pub fn run_for_file<T: Display>(year: u32, day: u32, solution: &str, file: &str, exec: impl Fn(&[String]) -> T) -> T {
    println!("-------------------");
    println!("-------------------");
    println!("{}", file);
    println!("-------------------");
    println!("-------------------");
    
    let lines = read_lines(year, day, file);
    println!();

    timed(&format!("{file} - {solution}"), || exec(&lines))
}

pub fn read_lines(year: u32, day: u32, file: &str) -> Vec<String> {
    let path = format!("./resources/y{year}/d{}/{file}", pad_left(day));

    println!("Reading file: {}", path);
    read_to_string(path).unwrap().lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn timed<T: Display>(name: &str, execute: impl Fn() -> T) -> T {
    let start = Instant::now();
    let result = execute();
    let end = start.elapsed();

    println!("[{name}] Result: {result}");
    println!("[{name}] Took {}ms\n", end.as_millis());

    result
}

fn pad_left(day: u32) -> String {
    format!("{:02}", day)
}
