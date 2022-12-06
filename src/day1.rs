use anyhow::Result;
use std::fs;
pub fn part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day1-input.txt")?;
    let mut elves: Vec<u32> = Vec::new();

    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for quantity in elf.split('\n') {
            let quantity = quantity.parse::<u32>().unwrap_or_default();
            sum += quantity;
        }
        elves.push(sum);
    }
    elves.sort();
    let result = elves.pop().unwrap_or_default();
    Ok(result)
}

pub fn part2() -> Result<u32> {
    let contents = fs::read_to_string("data/day1-input.txt")?;
    let mut elves: Vec<u32> = Vec::new();
    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for quantity in elf.split('\n') {
            let quantity = quantity.parse::<u32>().unwrap_or(0);
            sum += quantity;
        }
        elves.push(sum);
    }
    elves.sort();
    Ok(elves.iter().take(3).sum())
}
