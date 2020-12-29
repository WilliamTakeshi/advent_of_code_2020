use std::error::Error;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let trees = input
        .lines()
        .enumerate()
        .map(|(idx, line)| &line[calc_mod(idx, line, 3) - 1..calc_mod(idx, line, 3)])
        .filter(|sqm| *sqm == "#")
        .count();
    writeln!(io::stdout(), "{:?}", trees)?;
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for (right, down) in slopes.into_iter() {
        let trees = input
            .lines()
            .enumerate()
            .filter(|(idx, _)| idx % down == 0)
            .map(|(_, line)| line)
            .enumerate()
            .map(|(idx, line)| &line[calc_mod(idx, line, right) - 1..calc_mod(idx, line, right)])
            .filter(|sqm| *sqm == "#")
            .count();

        writeln!(io::stdout(), "{:?}", trees)?;
        result *= trees;
    }

    writeln!(io::stdout(), "{:?}", result)?;
    Ok(())
}

fn calc_mod(row: usize, tree_geology: &str, steps_right: usize) -> usize {
    ((steps_right * row) % tree_geology.len()) + 1
}
