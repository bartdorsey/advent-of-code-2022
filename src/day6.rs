use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs;

pub fn part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day6-input.txt")?;
    let mut last_four: Vec<char> = vec![];
    for (index, char) in contents.chars().enumerate() {
        last_four.push(char);
        if last_four.len() == 4 {
            let set: HashSet<&char> = HashSet::from_iter(last_four.iter());
            if set.len() == 4 {
                return Ok((index + 1) as u32);
            } else {
                last_four.remove(0);
            }
        }
    }

    Err(anyhow!("Couldn't find it"))
}

pub fn part2() -> Result<u32> {
    let contents = fs::read_to_string("data/day6-input.txt")?;
    let mut last_four: Vec<char> = vec![];
    for (index, char) in contents.chars().enumerate() {
        last_four.push(char);
        if last_four.len() == 14 {
            let set: HashSet<&char> = HashSet::from_iter(last_four.iter());
            if set.len() == 14 {
                return Ok((index + 1) as u32);
            } else {
                last_four.remove(0);
            }
        }
    }

    Err(anyhow!("Couldn't find it"))
}
