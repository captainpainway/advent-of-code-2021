use parse_input;

// https://adventofcode.com/2021/day/2

fn main() {
    let input_string = parse_input::parse("input.txt");
    println!("{}", part_one(&input_string));
    println!("{}", part_two(&input_string));
}

fn forward(directions: &Vec<(&str, i32)>) -> i32 {
    directions.iter()
        .fold(0, |acc, x| {
            if x.0 == "forward" {
                return acc + x.1;
            }
            acc
        })
}

fn depth(directions: &Vec<(&str, i32)>) -> i32 {
    directions.iter()
        .fold(0, |acc, x| {
            return match x.0 {
                "down" => acc + x.1,
                "up" => acc - x.1,
                _ => acc
            }
        })
}

fn aim_depth(directions: &Vec<(&str, i32)>) -> i32 {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    for x in directions {
        match x.0 {
            "down" => aim += x.1,
            "up" => aim -= x.1,
            "forward" => depth += aim * x.1,
            _ => ()
        }
    }
    depth
}

fn parse_directions(input: &Vec<String>) -> Vec<(&str, i32)> {
    input.iter()
        .map(|x| x.split(" ")
            .collect::<Vec<&str>>()
        )
        .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
        .collect()
}

fn part_one(input: &Vec<String>) -> i32 {
    let directions = parse_directions(input);
    forward(&directions) * depth(&directions)
}

fn part_two(input: &Vec<String>) -> i32 {
    let directions = parse_directions(input);
    forward(&directions) * aim_depth(&directions)
}