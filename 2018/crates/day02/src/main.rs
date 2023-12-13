use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    let mut boxes_with_double: u32 = 0;
    let mut boxes_with_triple: u32 = 0;
    for line in input.lines() {
        let mut box_has_double = false;
        let mut box_has_triple = false;
        let mut letter_counts = HashMap::new();
        for letter in line.chars() {
            *letter_counts.entry(letter).or_insert(0) += 1;
        }
        for &count in letter_counts.values() {
            if count == 2 {
                box_has_double = true;
            }
            if count == 3 {
                box_has_triple = true;
            }
        }
        if box_has_double {
            boxes_with_double += 1;
        }
        if box_has_triple {
            boxes_with_triple += 1;
        }
    }
    boxes_with_double * boxes_with_triple
}

fn solve_p2(input: &str) -> String {
    let mut result = String::new();
    let mut boxes: HashSet<&str> = HashSet::new();
    for box_id in input.lines() {
        for &second_box_id in &boxes {
            let mut differences: u32 = 0;
            for (first_box_letter, second_box_letter) in box_id.chars().zip(second_box_id.chars()) {
                if &first_box_letter != &second_box_letter {
                    differences += 1;
                }
            }
            if differences == 1 {
                for (first_box_letter, second_box_letter) in
                    box_id.chars().zip(second_box_id.chars())
                {
                    if first_box_letter == second_box_letter {
                        result.push(first_box_letter);
                    }
                }
                print!("\n");
            }
        }
        boxes.insert(box_id);
    }
    result
}
