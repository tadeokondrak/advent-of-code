use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
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
    println!("Checksum: {}", boxes_with_double * boxes_with_triple)
}
