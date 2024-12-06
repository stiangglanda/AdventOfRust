use std::fs;
use std::collections::{HashMap, HashSet};

pub fn day5() {
    let text = fs::read_to_string("assets/day5.txt")
        .expect("Should have been able to read the file");

    let data: Vec<String> = text.lines().map(String::from).collect();

    part1(&data);
    part2(&data);
}

fn extract_data(data: &Vec<String>) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let sep = data.iter().position(|line| line.is_empty()).unwrap();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in &data[..sep] {
        let parts: Vec<i32> = line.split('|').map(|x| x.parse().unwrap()).collect();
        rules.entry(parts[0]).or_insert_with(HashSet::new).insert(parts[1]);
    }

    let updates: Vec<Vec<i32>> = data[sep+1..].iter()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_valid(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    for i in 0..update.len() {
        for j in (i+1)..update.len() {
            if !rules.get(&update[i]).map_or(false, |rule_set| rule_set.contains(&update[j])) {
                return false;
            }
        }
    }
    true
}

fn fix_update(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut filtered_rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    for &i in update {
        if let Some(rule_set) = rules.get(&i) {
            let filtered_set: HashSet<i32> = rule_set.intersection(&update.iter().cloned().collect()).cloned().collect();
            filtered_rules.insert(i, filtered_set);
        }
    }

    let mut ordered_keys: Vec<i32> = filtered_rules.keys().cloned().collect();
    ordered_keys.sort_by(|a, b|
        filtered_rules.get(b).unwrap().len().cmp(&filtered_rules.get(a).unwrap().len())
    );

    ordered_keys
}

fn part1(data: &Vec<String>) -> i32 {
    let (rules, updates) = extract_data(data);
    let mut add_up = 0;

    for update in &updates {
        if is_valid(&rules, update) {
            add_up += update[update.len() / 2];
        }
    }

    println!("Part 1: {}", add_up);
    add_up
}

fn part2(data: &Vec<String>) -> i32 {
    let (rules, updates) = extract_data(data);
    let mut add_up = 0;

    for update in &updates {
        if !is_valid(&rules, update) {
            let fixed_update = fix_update(&rules, update);
            add_up += fixed_update[update.len() / 2];
        }
    }

    println!("Part 2: {}", add_up);
    add_up
}