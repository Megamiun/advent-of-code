extern crate core;
extern crate regex;
extern crate derive_more;
extern crate rustc_hash;
extern crate ibig;
extern crate forward_ref_generic;

use std::fmt::Display;
use std::fs::read_to_string;

pub mod y2023;
pub mod y2024;
mod util;

pub fn run_for_groups<T: Display>(year: u32, day: u32, files: &[&str], exec: &dyn Fn(&[&[String]]) -> T) {
    run_for_files(year, day, files, &|lines| {
        let groups = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
        exec(groups.as_slice())
    })
}

pub fn run_for_files<T: Display>(year: u32, day: u32, files: &[&str], exec: &dyn Fn(&[String]) -> T) {
    let padded_day = pad_left(day);

    println!("Running for {}-{}:", year, padded_day);

    files.iter().for_each(|file| {
        println!("-------------------");
        println!("{}", file);
        println!("-------------------");
        println!("-------------------");

        let lines = read_lines(year, &padded_day, file.to_string());

        println!();
        let result = exec(&lines);
        println!("\nResult: {}\n", result);
    })
}

pub fn run_for_files_with_postfix<T: Display>(
    year: u32,
    day: u32,
    files: &[&str],
    postfix: &str,
    exec: &dyn Fn(&[String], &[String]) -> T,
) {
    let padded_day = pad_left(day);

    println!("Running for {}-{}:", year, padded_day);

    files.iter().for_each(|file| {
        println!("-------------------");
        println!("{}", file);
        println!("-------------------");
        println!("-------------------");

        let lines = read_lines(year, &padded_day, file.to_string());
        let lines2 = read_lines(year, &padded_day, file.to_string() + "-" + postfix);

        println!();
        let result = exec(&lines, &lines2);
        println!("\nResult: {}\n", result);
    })
}

pub fn read_lines(year: u32, day: &String, file: String) -> Vec<String> {
    let path = "./resources/y".to_string() + &year.to_string() + "/d" + day + "/" + &file;

    println!("Reading file: {}", path);
    read_to_string(path).unwrap().lines()
        .map(|line| line.to_string())
        .collect()
}

fn pad_left(day: u32) -> String {
    format!("{:02}", day)
}
