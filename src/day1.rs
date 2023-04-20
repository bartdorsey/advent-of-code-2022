use std::fs;
use anyhow::{anyhow, Result};

pub fn part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day1-input.txt")?;
    let mut elves: Vec<_> = contents
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .sum()
        })
        .collect();
    elves.sort();
    let result = elves.pop().ok_or(anyhow!("Couldn't pop elf"))?;
    Ok(result)
}

pub fn part2() -> Result<u32> {
    let contents = fs::read_to_string("data/day1-input.txt")?;
    let mut elves: Vec<u32> = Vec::new();
    for elf in contents.split("\n\n") {
        let mut sum = 0;
        for quantity in elf.split('\n') {
            let quantity = quantity.parse().unwrap_or(0);
            sum += quantity;
        }
        elves.push(sum);
    }
    elves.sort();
    Ok(elves.iter().take(3).sum())
}
