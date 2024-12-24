extern crate core;
extern crate derive_more;
extern crate forward_ref_generic;
extern crate ibig;
extern crate itertools;
extern crate regex;
extern crate rustc_hash;

use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;
use itertools::Itertools;

pub mod y2023;
pub mod y2024;
pub mod util;

pub fn run_for_groups<T: Display>(year: u32, day: u32, solution: &str, files: &[&str], exec: impl Fn(&[&[String]]) -> T) {
    run_for_files(year, day, solution, files, |lines| {
        let groups = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
        exec(groups.as_slice())
    })
}

pub fn run_for_n_groups<T: Display, const N: usize>(year: u32, day: u32, solution: &str, files: &[&str], exec: impl Fn(&[&[String]; N]) -> T) {
    run_for_files(year, day, solution, files, |lines| {
        let groups = lines.splitn(N, |line| line.is_empty()).collect_vec();
        exec(&groups.try_into().unwrap())
    })
}

pub fn run_for_files<T: Display>(year: u32, day: u32, solution: &str, files: &[&str], exec: impl Fn(&[String]) -> T) {
    files.iter().for_each(|file| {
        let key = format!("{file} - {solution}");
        println!("-------------------");
        println!("{key}");
        println!("-------------------");
        println!("-------------------");

        let lines = read_lines(year, day, file);

        println!();
        
        timed(&key, || exec(&lines));
    })
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
