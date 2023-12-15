use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    input.trim().split(",").map(|x| hash(x) as u32).sum()
}

fn solve_p2(input: &str) -> u64 {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    for x in input.trim().split(",") {
        if let Some((name, rest)) = x.split_once("=") {
            let box_index = hash(name);
            let value = rest.parse().unwrap();
            let the_box = &mut boxes[usize::from(box_index)];
            if let Some(i) = the_box.iter().position(|(n, _v)| n == name) {
                the_box[i] = (name.to_owned(), value);
            } else {
                the_box.push((name.to_owned(), value));
            }
        } else if let Some(name) = x.strip_suffix("-") {
            let box_index = hash(name);
            let the_box = &mut boxes[usize::from(box_index)];
            if let Some(i) = the_box.iter().position(|(n, _v)| n == name) {
                the_box.remove(i);
            }
        }
    }
    let mut total = 0;
    for (i, the_box) in boxes.iter().enumerate() {
        for (j, (_, focal_length)) in the_box.iter().enumerate() {
            total += (i + 1) * (j + 1) * *focal_length as usize;
        }
    }
    total as u64
}

fn hash(x: &str) -> u8 {
    x.bytes()
        .fold(0u8, |acc, c| acc.wrapping_add(c).wrapping_mul(17))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
