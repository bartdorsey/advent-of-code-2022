use anyhow::Result;
use std::{fs, vec};

pub fn part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day4-input.txt").unwrap();
    let count = contents
        .lines()
        .map(|line| {
            line.split(&[',', '-'])
                .map(|char| char.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|line| {
            if let [e1_start, e1_end, e2_start, e2_end] = &line[..] {
                e1_start <= e2_start && e1_end >= e2_end || e2_start <= e1_start && e2_end >= e1_end
            } else {
                false
            }
        })
        .count() as u32;
    Ok(count)
}

pub fn part2() -> Result<u32> {
    let mut count = 0;
    let contents = fs::read_to_string("data/day4-input.txt").unwrap();
    let lines = contents.lines();
    let splits = lines.map(|line| line.split(',').collect::<Vec<&str>>());
    for pair in splits {
        let mut e = vec![];
        for elf in pair {
            let elf = elf.split('-');
            let elf = elf.into_iter().map(|elf| elf.parse::<u32>().unwrap());
            e.push(elf.collect::<Vec<u32>>());
        }
        let e1_start = e[0][0];
        let e1_end = e[0][1];
        let e2_start = e[1][0];
        let e2_end = e[1][1];
        if e1_end >= e2_start && e2_end >= e1_start {
            count += 1;
        }
    }
    Ok(count)
}
