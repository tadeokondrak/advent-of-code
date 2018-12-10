use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
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
                        print!("{}", first_box_letter);
                    }
                }
                print!("\n");
            }
        }
        boxes.insert(box_id);
    }
}
