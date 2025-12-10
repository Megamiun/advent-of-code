use crate::util::parse_num::parse_usize;
use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;
use std::iter::once;

#[derive(Debug)]
struct Configuration {
    lights: Vec<Light>,
    buttons: Vec<Vec<usize>>
}

#[derive(Debug)]
struct Light {
    state: bool,
    joltage: usize
}

#[allow(dead_code)]
pub fn get_minimum_presses_lights(lines: &[String]) -> usize {
    get_configurations(lines).iter().map(get_min_presses_lights).sum()
}

#[allow(dead_code)]
pub fn get_minimum_presses_joltage(lines: &[String]) -> usize {
    get_configurations(lines).iter().map(get_min_presses_joltage).sum()
}

fn get_min_presses_lights(config: &Configuration) -> usize {
    let expected = config.lights.iter().map(|light| light.state).collect_vec();

    let mut to_visit = VecDeque::new();

    to_visit.push_back((vec![false; expected.len()], vec![]));

    while let Some((lights, pressed)) = to_visit.pop_front() {
        let max = match pressed.last() {
            Some(last) =>  last + 1,
            None => 0
        };

        for next in max..config.buttons.len() {
            let button = &config.buttons[next];
            let mut new_lights = lights.clone();
            for light in button {
                new_lights[*light] = !new_lights[*light]
            }

            if new_lights == expected {
                return pressed.len() + 1
            }

            let mut new_presses = pressed.clone();
            new_presses.push(next);

            to_visit.push_back((new_lights, new_presses));
        }
    }

    panic!("No solution found")
}

fn get_min_presses_joltage(config: &Configuration) -> usize {
    let expected = config.lights.iter()
        .map(|light| light.joltage)
        .collect_vec();

    get_min_presses_joltage_rec(&expected, &config.buttons)
        .unwrap_or_else(|| panic!("No solution found"))
}

fn get_min_presses_joltage_rec(missing_joltage: &Vec<usize>, buttons: &[Vec<usize>]) -> Option<usize> {
    if buttons.is_empty() {
        if missing_joltage.iter().all(|j| *j == 0) {
            return Some(0);
        }

        return None
    }

    let (clicks, buttons_to_manage) = missing_joltage.iter().enumerate()
        .filter(|(_, joltage)| **joltage != 0)
        .map(|(idx, joltage)| (
            *joltage,
            buttons.iter()
                .filter(|button| button.contains(&idx))
                .sorted_by_key(|buttons| buttons.len()).rev()
                .collect_vec()
        ))
        .min_by_key(|(_, buttons)| buttons.len())
        .unwrap();

    let mut results = vec![];

    'combination: for combination in get_next(buttons_to_manage.len(), clicks) {
        let mut new_missing_joltage = missing_joltage.clone();

        for (times, &button) in combination.iter().zip(buttons_to_manage.iter()) {
            for effect in button {
                if new_missing_joltage[*effect] < *times {
                    continue 'combination
                }
                new_missing_joltage[*effect] = new_missing_joltage[*effect] - times
            }
        }

        let next_buttons = buttons.iter()
            .filter(|button| button.iter().all(|idx| new_missing_joltage[*idx] > 0)).cloned()
            .collect_vec();

        if let Some(result) = get_min_presses_joltage_rec(&new_missing_joltage, &next_buttons) {
            results.push(result);
            // return Some(result + clicks)
        }
    }

    // TODO Investigate how to make sure the min pos is always 0
    if let Some(min_pos) = results.iter().position_min().filter(|res| *res != 0) {
        println!("Joltage: {:?}", missing_joltage);
        println!("Buttons: {:?}", buttons);
        println!("Buttons to manage: {:?}", buttons_to_manage);
        println!("Results: {:?}", results);
        println!("Min position: {:?}", min_pos);
        println!()
    }
    results.iter().min().map(|r| r + clicks)
}

fn get_next(buttons: usize, clicks: usize) -> Vec<Vec<usize>> {
    match buttons {
        0 => vec![],
        1 => vec![vec![clicks]],
        _ => (0..=clicks).rev().flat_map(|curr|
            get_next(buttons - 1, clicks - curr).iter()
                .map(|i| once(curr).chain(i.iter().cloned()).collect_vec())
                .collect_vec()
        ).collect_vec(),
    }
}

fn get_configurations(lines: &[String]) -> Vec<Configuration> {
    let lights_regex = Regex::new(r"\[(.*)]").unwrap();
    let buttons_regex = Regex::new(r"\((.*?)\)").unwrap();
    let joltage_regex = Regex::new(r"\{(.*)}").unwrap();

    lines.iter().map(|line| {
        let light_states = lights_regex
            .captures_iter(line)
            .flat_map(|lights| lights.get(1).unwrap().as_str().chars().map(|c| c == '#'));
        let light_joltages = joltage_regex
            .captures_iter(line)
            .flat_map(|joltages| joltages.get(1).unwrap().as_str().split(",").map(|joltage| parse_usize(joltage)));

        let lights = light_states.zip(light_joltages)
            .map(|(state, joltage)| Light { state, joltage })
            .collect_vec();

        let buttons = buttons_regex
            .captures_iter(line)
            .map(|button| button.get(1).unwrap().as_str().split(",").map(|light| parse_usize(light)).collect_vec())
            .collect_vec();

        Configuration { lights, buttons }
    }).collect_vec()
}
