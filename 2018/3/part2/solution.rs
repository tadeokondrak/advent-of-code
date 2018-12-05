#[allow(dead_code)] // id isn't used in part 1
struct Claim {
    id: u32,
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}

fn main() {
    let input = include_str!("../input");
    let mut grid = [[0u32; 1000]; 1000];
    let mut claims = vec![];
    for line in input.lines() {
        let claim = parse_line(line);
        claims.push(claim);
    }
    for claim in &claims {
        for x in claim.x1..claim.x2 {
            for y in claim.y1..claim.y2 {
                grid[x as usize][y as usize] += 1;
            }
        }
    }
    for claim in claims {
        let mut uncontested = true;
        for x in claim.x1..claim.x2 {
            for y in claim.y1..claim.y2 {
                if grid[x as usize][y as usize] > 1 {
                    uncontested = false;
                }
            }
        }
        if uncontested == true {
            println!("Non-overlapping claim: {}", claim.id)
        }
    }
}

fn parse_line(line: &str) -> Claim {
    let split: Vec<&str> = line.split(' ').collect();
    let claimid: u32 = split[0][1..].parse().unwrap();
    let coords_str = &split[2][..&split[2].len()-1];
    let coords: Vec<u32> = coords_str.split(',')
        .map(|num| {
            num.parse::<u32>().unwrap()
        }).collect();
    let size: Vec<u32> = split[3].split('x')
        .map(|num| {
            num.parse::<u32>().unwrap()
        }).collect();
    return Claim {
        id: claimid,
        x1: coords[0],
        y1: coords[1],
        x2: coords[0] + size[0],
        y2: coords[1] + size[1]
    };
}
