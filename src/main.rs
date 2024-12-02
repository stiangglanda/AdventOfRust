use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    // --snip--
    println!("In file foo.txt");

    let lists= get_lists("./foo.txt")?;
    println!("total_distance: {}",  get_total_distance(&lists));
    println!("total_similarity_score: {}",  get_total_similarity_score(&lists));

    Ok(())

}

fn get_lists<P>(filename: P) -> Result<(Vec<u32>, Vec<u32>), io::Error> 
where P: AsRef<Path>, {
    let mut list1:Vec<u32>=Vec::new();
    let mut list2:Vec<u32>=Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines().flatten() {
        let list:Vec<u32>=line.split_whitespace().map(|s| s.parse::<u32>().expect("i expect there to be only u32 numbers")).collect();
        if !list.is_empty()
        {
            list1.push(*list.first().expect("there should be two values here"));
            list2.push(*list.last().expect("there should be two values here"));
        }
    }
    list1.sort();
    list2.sort();
    Ok((list1,list2))
}

fn get_total_distance(lists:&(Vec<u32>, Vec<u32>)) -> u32 {
    lists.0.iter().zip(lists.1.iter()).map(|(&x,&y)| x.abs_diff(y)).sum()
}

fn get_total_similarity_score(lists:&(Vec<u32>, Vec<u32>)) -> u32 {
    let mut result:u32=0;
    for list in lists.0.iter() {
        result += list * lists.1.iter().filter(|&x|x==list).count() as u32;
    }
    result
}