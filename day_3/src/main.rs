use parse_input;

// https://adventofcode.com/2021/day/3

fn main() {
    let input_string = parse_input::parse("input.txt");
    println!("{}", part_one(&input_string));
    //println!("{}", part_two(&input_string));
}

fn part_one(input: &Vec<String>) -> isize {
    let len = input[0].len();
    let mut gamma = "".to_string();

    for y in 0..len {
        let mut tmp_string = "".to_owned();
        for x in input {
            tmp_string = tmp_string + &((x.as_bytes()[y] as char).to_string());
        }

        let zeroes = tmp_string.matches("0").count();
        let ones = tmp_string.matches("1").count();

        if zeroes > ones {
            gamma = gamma.to_owned() + "0"
        } else {
            gamma = gamma.to_owned() + "1"
        }
    }

    let epsilon: String = gamma.chars().map(|x| match x {
        '0' => '1',
        '1' => '0',
        _ => x
    }).collect();

    let gamma_num = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = isize::from_str_radix(&epsilon, 2).unwrap();

    gamma_num * epsilon_num
}
