use std::fs;

type InputGrid = Vec<Vec<u32>>;
type Coordinate = (i32, i32);

fn main() {

    let path: &str = "./input";

    let (input, start): (InputGrid, Coordinate) = parse_input(path);

    //println!("{:?}", input);

    let ans1: usize = part1(input, start);
    println!("{}", ans1);
}

fn parse_input(path: &str) -> (InputGrid, Coordinate) {
    let input_raw: String = fs::read_to_string(path).expect("failed reading input");
    let mut start_position: Coordinate = (0, 0);
    let mut input: InputGrid = Vec::new();

    for (y, line) in input_raw.split("\n").enumerate() {
        if line.contains("^") {
            let x = line.find("^").unwrap();
            start_position = (x as i32, y as i32);
        }

        let mut newline = Vec::new();
        line.chars()
            .for_each(|x| {
                match x {
                    '.' => newline.push(0),
                    '#' => newline.push(1),
                    '^' => newline.push(0),
                    _ => {},
                }
            });
        
        if newline.len() > 0 {
            input.push(newline);
        }
        
    }
    
    (input, start_position)
}


fn part1(input: InputGrid, start: Coordinate) -> usize {
    let mut direction = (0, -1); // (x, y), + is down/right
    let mut seen: Vec<Coordinate> = Vec::new();
    let mut current_position: Coordinate = start;
    let length: (i32, i32) = (input.len() as i32, input[0].len() as i32);

    loop {
        if !seen.contains(&current_position) {
            seen.push(current_position);
        }

        if (current_position.0 + direction.0 == length.0) | (current_position.1 + direction.1 == length.1) {
            break
        }
        
        if input[(current_position.1 + direction.1) as usize][(current_position.0 + direction.0) as usize] == 1 {
            direction = rotate(direction);
        }
        current_position = (current_position.0 + direction.0, current_position.1 + direction.1);
    }

    let visited = seen.len();

    visited
}

fn rotate(mut direction: Coordinate) -> Coordinate {
    if direction.0 == 0 {
        direction = (-direction.1, -direction.0);
    } else {
        direction = (direction.1, direction.0);
    }
    direction
}
