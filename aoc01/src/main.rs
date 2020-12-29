use std::io::{self, Read, Write};
use std::error::Error;
use std::collections::HashSet;
use itertools::Itertools;


fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input = parse_input(&input);

    // println!("{:?}", input);
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn parse_input(input: &str) -> HashSet<i32>{
    input.lines()
        .map(|line| line.parse().expect("could not parse a number"))
        .collect()
}


fn part1(input: &HashSet<i32>) -> Result<(), Box<dyn Error>> {
    for val in input.iter() {
        match input.get(&(2020-val)) {
            None => continue,
            Some(v) => {
                writeln!(io::stdout(), "{}", v * val)?;
                return Ok(());
            }
        }
    };
    Ok(())
}

fn part2(input: &HashSet<i32>) -> Result<(), Box<dyn Error>> {
    let combinations = input.into_iter().combinations_with_replacement(2);
    for combination in combinations {
        match input.get(&(2020-(combination[0] + combination[1]))) {
            None => continue,
            Some(v) => {
                writeln!(io::stdout(), "{}", v * combination[0] * combination[1])?;
                return Ok(());
            }
        }
    };

    Ok(())
}