use std::io::{stdin, BufRead};
use std::{collections::HashMap, iter::once};

fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" | "iyr" | "eyr" => {
            value.len() == 4
                && value.parse::<u32>().map_or(false, |n| match key {
                    "byr" => n >= 1920 && n <= 2002,
                    "iyr" => n >= 2010 && n <= 2020,
                    "eyr" => n >= 2020 && n <= 2030,
                    _ => unreachable!(),
                })
        }
        "hgt" => {
            value.len() >= 2
                && match value[..value.len() - 2].parse::<u32>() {
                    Ok(n) if value.ends_with("cm") && n >= 150 && n <= 193 => true,
                    Ok(n) if value.ends_with("in") && n >= 59 && n <= 76 => true,
                    _ => false,
                }
        }
        "hcl" => {
            value.len() == 7
                && value
                    .strip_prefix('#')
                    .map_or(false, |rest| rest.chars().all(|c| c.is_digit(16)))
        }
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => unreachable!(),
    }
}

fn main() {
    let mut map = HashMap::<String, String>::new();
    let (mut p1, mut p2) = (0, 0);
    for line in stdin().lock().lines().chain(once(Ok(String::new()))) {
        let line = line.unwrap();

        if line.is_empty() {
            let has_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|&field| map.contains_key(field));
            let fields_are_valid = map.iter().all(|(k, v)| validate(k, v));
            p1 += has_fields as u32;
            p2 += (has_fields && fields_are_valid) as u32;
            map.clear();
            continue;
        }

        for ent in line.split(" ") {
            let mut it = ent.split(':');
            let k = it.next().unwrap();
            let v = it.next().unwrap();
            map.insert(k.to_string(), v.to_string());
        }
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
