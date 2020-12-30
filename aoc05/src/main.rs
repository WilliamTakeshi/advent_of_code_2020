use std::error::Error;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // part1(&input)?;
    part2(&input)?;
    Ok(())
}


fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let max = input.lines()
        .map(|val| val.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1"))
        .map(|bin| isize::from_str_radix(&bin, 2).unwrap())
        .max();

    writeln!(io::stdout(), "{:?}", max)?;

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut vec = input.lines()
        .map(|val| val.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1"))
        .map(|bin| isize::from_str_radix(&bin, 2).unwrap())
        .collect::<Vec<isize>>();

    vec.sort();

    let zip_with_next: Vec<(isize, isize)> = vec.iter()
        .zip(vec.iter().skip(1))
        .map(|(&cur, next)| (cur, next-cur))
        .filter(|(_, diff)| (diff > &1))
        .collect();


    writeln!(io::stdout(), "{:?}", zip_with_next[0].0+1)?;

    Ok(())
}