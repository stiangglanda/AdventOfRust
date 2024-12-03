use std::fs;
use regex::Regex;

pub fn day3() {
    let text = fs::read_to_string("assets/day3.txt")
        .expect("Should have been able to read the file");
    let re = Regex::new( r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all matches
    let mut result=0;

    for captures in re.captures_iter(text.as_str()) {
        let num0:i32 = captures[1].parse().expect("First number");
        let num1:i32 = captures[2].parse().expect("Second number");

        result += num0 * num1;
    }

    println!("Result: {}", result);
}
