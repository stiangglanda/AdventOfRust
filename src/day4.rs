use std::fs;
use regex::Regex;

pub fn day4() {
    let text = fs::read_to_string("assets/day4.txt")
        .expect("Should have been able to read the file");

    part1(&text);
    part2(&text);
}

fn part1(text: &String) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all matches
    let mut result = 0;

    for captures in re.captures_iter(text.as_str()) {
        let num0: i32 = captures[1].parse().expect("First number");
        let num1: i32 = captures[2].parse().expect("Second number");

        result += num0 * num1;
    }

    println!("Result: {}", result);
}

fn part2(text: &String) {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    // Find all matches
    let mut result: i32 = 0;
    let mut enabled: bool = true;

    for captures in re.captures_iter(text.as_str()) {
        match captures.get(0).map(|m| m.as_str()).unwrap_or("") {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            s if s.starts_with("mul(") && enabled => {
                result += captures[2].parse::<i32>().expect("First number") *
                    captures[3].parse::<i32>().expect("Second number");
            },
            _ => {}
        }
    }

    println!("Result: {}", result);
}
