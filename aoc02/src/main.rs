use std::io::{self, Read, Write};
use std::error::Error;

fn main()  -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}


fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let valid_pws = input.lines()
        .map(|line| pw_is_valid_pt1(line))
        .filter(|b| *b)
        .count();
    writeln!(io::stdout(), "{}", valid_pws)?;
    Ok(())
}


fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let valid_pws = input.lines()
        .map(|line| pw_is_valid_pt2(line))
        .filter(|b| *b)
        .count();
    writeln!(io::stdout(), "{}", valid_pws)?;
    Ok(())
}


fn pw_is_valid_pt1(line: &str) -> bool {
    let vec: Vec<&str> = line.split(|c| c == ' ' || c == '-' || c == ':').collect();
    let min: usize = vec[0].parse().unwrap(); // Potential panic!
    let max: usize = vec[1].parse().unwrap(); // Potential panic!
    let letter: char = vec[2].parse().unwrap(); // Potential panic!
    let pw = vec[4]; // Potential panic!


    let count: usize = pw.chars()
        .filter(|c| *c == letter)
        .count();
    
    
    (count >= min) & (count <= max)
}

fn pw_is_valid_pt2(line: &str) -> bool {
    let vec: Vec<&str> = line.split(|c| c == ' ' || c == '-' || c == ':').collect();
    let pos1: usize = vec[0].parse().unwrap(); // Potential panic!
    let pos2: usize = vec[1].parse().unwrap(); // Potential panic!
    let letter: char = vec[2].parse().unwrap(); // Potential panic!
    let pw = vec[4]; // Potential panic!

    let pos1_contains = pw.chars().nth(pos1 - 1).unwrap() == letter; // Potential panic!
    let pos2_contains = pw.chars().nth(pos2 - 1).unwrap() == letter; // Potential panic!
    
    (pos1_contains & !pos2_contains) | (!pos1_contains & pos2_contains)
}