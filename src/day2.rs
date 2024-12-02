use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day2() {
    let reports=get_reports("assets/day2.txt").expect("Failed to get report list");
    println!("day 2:");
    println!("count_safe: {}",count_safe(&reports));
    println!("count_safe_with_tolerance: {}",count_safe_with_tolerance(&reports));
}
fn get_reports<P>(filename: P) -> Result<Vec<Vec<u32>>, io::Error>
where P: AsRef<Path>, {
    let mut reports:Vec<Vec<u32>>=Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines().flatten() {
        let list:Vec<u32>=line.split_whitespace().map(|s| s.parse::<u32>().expect("i expect there to be only u32 numbers")).collect();
        reports.push(list);
    }
    Ok(reports)
}

fn count_safe(reports:&Vec<Vec<u32>>) -> u32 {
    let mut count:u32=0;
    for report in reports {
        if is_sorted(&report) && is_difference_in_bounds(&report)
        {
            count+=1;
        }
    }
    count
}

fn count_safe_with_tolerance(reports:&Vec<Vec<u32>>) -> u32 {
    let mut count:u32=0;
    for report in reports {

        if is_sorted(&report) && is_difference_in_bounds(&report)
        {
            count+=1;
        }
        else {
            for i in 0..report.len() {
                let mut report_copy= report.clone();
                report_copy.remove(i);
                if is_sorted(&report_copy) && is_difference_in_bounds(&report_copy){
                    count += 1;
                    break
                }
            }
        }
    }
    count
}

fn is_difference_in_bounds(report: &Vec<u32>) -> bool {
    for i in 0..report.len() -1 {
        if i <= report.len() -2 {
            match report[i].abs_diff(report[i+1]) {
                1..=3 => continue,
                _ => return false
            }
        }
    }
    true
}

fn is_sorted(report: &Vec<u32>) -> bool {
    let mut report_reverse = report.clone();
    report_reverse.reverse();

    report_reverse.is_sorted() || report.is_sorted()
}