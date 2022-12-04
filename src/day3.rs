use anyhow::Result;
use std::{collections::HashSet, fs};

pub fn day3_part1() -> Result<u32> {
    let contents = fs::read_to_string("data/day3-input.txt").unwrap();
    let mut sum = 0;
    for line in contents.lines() {
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let set1: HashSet<char> = comp1.chars().collect();
        let set2: HashSet<char> = comp2.chars().collect();
        let intersection = set1.intersection(&set2);

        let mut subtotal = 0;
        for value in intersection {
            let d = *value as u32;
            if value.is_uppercase() {
                let v = d - 64 + 26;
                subtotal += v;
            } else {
                let v = d - 96;
                subtotal += v;
            }
        }
        sum += subtotal;
    }
    Ok(sum)
}

pub fn day3_part2() -> Result<u32> {
    let contents = fs::read_to_string("data/day3-input.txt").unwrap();
    let mut sum = 0;
    for [chunk1, chunk2, chunk3] in contents.lines().array_chunks() {
        let set1: HashSet<char> = chunk1.chars().collect();
        let set2: HashSet<char> = chunk2.chars().collect();
        let set3: HashSet<char> = chunk3.chars().collect();
        let i1: HashSet<&char> = set1.intersection(&set2).collect();
        let i2: HashSet<&char> = set2.intersection(&set3).collect();
        let i3 = i1.intersection(&i2);

        let mut subtotal = 0;
        for value in i3 {
            let d = **value as u32;
            if value.is_uppercase() {
                let v = d - 64 + 26;
                subtotal += v;
            } else {
                let v = d - 96;
                subtotal += v;
            }
        }
        sum += subtotal;
    }
    Ok(sum)
}
