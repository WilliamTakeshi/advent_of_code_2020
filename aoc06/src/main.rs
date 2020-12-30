use std::collections::HashSet;
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
    let result: usize = input
        .split("\n\n")
        .map(|form| {
            form.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();

    writeln!(io::stdout(), "{:?}", &result);

    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let result: usize = input
        .split("\n\n")
        .map(|form| {
            form.split('\n')
                .map(|form| form.chars().collect::<HashSet<_>>())
                .fold(
                    "abcdefghijklmnopqrstuvwxyz"
                        .chars()
                        .collect::<HashSet<char>>(),
                    |acc, b| acc.intersection(&b).cloned().collect::<HashSet<char>>(),
                )
                .len()
        })
        .sum();

    writeln!(io::stdout(), "{:?}", &result);

    Ok(())
}

// fn part1(input: &str) -> Result<(), Box<dyn Error>> {
//     let mut result = 0;
//     let forms: Vec<&str> = input.split("\n\n").collect();

//     for form in forms.into_iter() {
//         let form_parsed: HashSet<char> = form.chars().filter(|c| c.is_alphanumeric()).collect();

//         let count = form_parsed.len();

//         result += count;
//     }

//     writeln!(io::stdout(), "{:?}", &result);

//     Ok(())
// }

// fn part2(input: &str) -> Result<(), Box<dyn Error>> {
//     let mut result = 0;
//     let forms: Vec<&str> = input.split("\n\n").collect();

//     for form in forms.into_iter() {
//         let form_parsed = form
//             .split('\n')
//             .map(|form| form.chars().collect::<HashSet<_>>())
//             .fold(
//                 "abcdefghijklmnopqrstuvwxyz"
//                     .chars()
//                     .collect::<HashSet<char>>(),
//                 |acc, b| acc.intersection(&b).cloned().collect::<HashSet<char>>(),
//             );

//         let count = form_parsed.len();
//         result += count;
//     }

//     writeln!(io::stdout(), "{:?}", &result);

//     Ok(())
// }
