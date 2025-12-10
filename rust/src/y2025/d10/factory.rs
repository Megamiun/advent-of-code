use crate::util::parse_num::{parse_u32, parse_usize};
use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

struct Configuration {
    lights: Vec<Light>,
    buttons: Vec<Vec<usize>>
}

#[derive(Debug)]
struct Light {
    state: bool,
    joltage: u32
}

struct State {
    lights: Vec<bool>,
    pressed: Vec<usize>
}

#[allow(dead_code)]
pub fn get_minimum_presses(lines: &[String]) -> usize {
    get_configurations(lines).iter().map(get_min_presses).sum()
}

fn get_min_presses(config: &Configuration) -> usize {
    let expected = config.lights.iter().map(|light| light.state).collect_vec();

    let mut to_visit = VecDeque::<State>::new();

    to_visit.push_back(State { lights: vec![false; expected.len()], pressed: vec![] });

    while let Some(State { lights, pressed }) = to_visit.pop_front() {
        let max = match pressed.last() {
            Some(last) =>  last + 1,
            None => 0
        };

        for next in max..config.buttons.len() {
            let button = &config.buttons[next];
            let mut new_lights = lights.clone();
            for light in button {
                new_lights[*light] = !new_lights[*light];
            }

            if new_lights == expected {
                return pressed.len() + 1;
            }

            let mut new_presses = pressed.clone();
            new_presses.push(next);

            to_visit.push_back(State { lights: new_lights, pressed: new_presses });
        }
    }

    0
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
            .flat_map(|joltages| joltages.get(1).unwrap().as_str().split(",").map(|joltage| parse_u32(joltage)));

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
