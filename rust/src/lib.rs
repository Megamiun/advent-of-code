extern crate core;
extern crate regex;

use std::fmt::Display;
use std::fs::read_to_string;

pub mod y2023;
pub mod y2024;

pub fn run_for_files<T: Display>(year: u32, day: u32, files: &[&str], exec: &dyn Fn(Vec<String>) -> T) {
    let padded_day = pad_left(day);

    println!("Running for {}-{}:", year, padded_day);

    files.iter().for_each(|file| {
        println!("-------------------");
        println!("{}", file);
        println!("-------------------");
        println!("-------------------");

        let lines = read_lines(year, &padded_day, file.to_string());

        println!();
        let result = exec(lines);
        println!("\nResult: {}\n", result);
    })
}

pub fn run_for_files_with_postfix<T: Display>(
    year: u32,
    day: u32,
    files: &[&str],
    postfix: &str,
    exec: &dyn Fn(Vec<String>, Vec<String>) -> T,
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
        let result = exec(lines, lines2);
        println!("\nResult: {}\n", result);
    })
}

pub fn read_lines(year: u32, day: &String, file: String) -> Vec<String> {
    let mut lines = Vec::new();

    let path = "./resources/y".to_string() + &year.to_string() + "/d" + day + "/" + &file;

    println!("Reading file: {}", path);
    for i in read_to_string(path).unwrap().lines() {
        lines.push(i.to_string());
    }

    lines
}

fn pad_left(day: u32) -> String {
    format!("{:02}", day)
}
