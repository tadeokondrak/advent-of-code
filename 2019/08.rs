use std::io::{stdin, Read};

fn main() {
    let mut image = Vec::new();
    stdin().read_to_end(&mut image).unwrap();
    image.pop();
    image.iter_mut().for_each(|p| *p -= b'0');

    let (width, height) = (25, 6);
    let layers = image.chunks(width * height).collect::<Vec<_>>();

    let p1 = layers
        .iter()
        .map(|layer| {
            (
                layer.iter().filter(|&&x| x == 0).count(),
                layer.iter().filter(|&&x| x == 1).count(),
                layer.iter().filter(|&&x| x == 2).count(),
            )
        })
        .min_by_key(|&(zeroes, _, _)| zeroes)
        .map(|(_, ones, twos)| ones * twos)
        .unwrap();

    println!("Part 1: {}", p1);

    let rendered = (0..width * height)
        .map(|i| layers.iter().find(|layer| layer[i] != 2).unwrap()[i])
        .collect::<Vec<_>>();

    println!("Part 2: ");
    (0..height).for_each(|y| {
        for x in 0..width {
            match rendered[x + y * width] {
                0 => print!("\x1B[48;5;0m "),
                1 => print!("\x1B[48;5;7m#"),
                _ => unreachable!(),
            }
        }
        println!("\x1B[0m");
    });
}
