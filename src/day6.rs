use std::fs;
use std::collections::{HashMap, HashSet};

pub fn day6() {
    let text = fs::read_to_string("assets/day6.txt")
        .expect("Should have been able to read the file");

    let data: Vec<String> = text.lines().map(String::from).collect();

    part1(&data);
    part2(&data);
}

fn get_guard_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return (i, j);
            }
        }
    }
    panic!("No guard found in map")
}

fn patrol(map: &Vec<Vec<char>>, pos: Option<(usize, usize)>, idx: Option<usize>) -> (bool, Option<HashSet<(usize, usize)>>, Option<HashMap<(usize, usize), ((usize, usize), usize)>>) {
    let directions = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let rows = map.len();
    let cols = map[0].len();

    let mut pos = pos.unwrap_or_else(|| get_guard_pos(map));
    let mut idx = idx.unwrap_or(0);

    let mut visited = HashSet::new();
    visited.insert(pos);

    let mut visited_entry = HashMap::new();

    loop {
        let d = directions[idx];
        let n = (
            if d.0 == usize::MAX { pos.0.wrapping_sub(1) } else { pos.0 + d.0 },
            if d.1 == usize::MAX { pos.1.wrapping_sub(1) } else { pos.1 + d.1 }
        );

        // Check map boundaries
        if n.0 >= rows || n.1 >= cols {
            return (true, Some(visited), Some(visited_entry));
        }

        // Hit wall
        if map[n.0][n.1] == '#' {
            idx = (idx + 1) % 4;
            continue;
        }

        visited.insert(n);

        if !visited_entry.contains_key(&n) {
            visited_entry.insert(n, (pos, idx));
        } else if visited_entry[&n] == (pos, idx) {
            return (false, None, None); // Loop detected
        }

        pos = n;
    }
}

fn part1(data: &Vec<String>) -> usize {
    let map: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();
    let (_, visited, _) = patrol(&map, None, None);
    visited.unwrap().len()
}

fn part2(data: &Vec<String>) -> usize {
    let map: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();
    let (_, visited, visited_entry) = patrol(&map, None, None);

    let visited = visited.unwrap();
    let visited_entry = visited_entry.unwrap();
    let guard_pos = get_guard_pos(&map);

    let mut loop_count = 0;

    for &(vi, vj) in &visited {
        if (vi, vj) == guard_pos {
            continue;
        }

        let mut map_copy = map.clone();
        map_copy[vi][vj] = '#';

        let (pos, idx) = visited_entry[&(vi, vj)];

        let (is_leaving, _, _) = patrol(&map_copy, Some(pos), Some(idx));
        if !is_leaving {
            loop_count += 1;
        }
    }

    println!("Part 1: {}", part1(data));
    println!("Part 2: {}", loop_count);
    loop_count
}